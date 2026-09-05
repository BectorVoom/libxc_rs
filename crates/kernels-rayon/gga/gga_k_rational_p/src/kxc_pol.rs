//! GGA_K_RATIONAL_P kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_rational_p.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_rational_p_kxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
    param_p: f64,
    param_C2: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_p = f64x8::splat(param_p);
    let param_C2 = f64x8::splat(param_C2);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_v2rho2_0 = V_ZERO;
        let mut acc_v2rho2_1 = V_ZERO;
        let mut acc_v2rho2_2 = V_ZERO;
        let mut acc_v2rhosigma_0 = V_ZERO;
        let mut acc_v2rhosigma_1 = V_ZERO;
        let mut acc_v2rhosigma_2 = V_ZERO;
        let mut acc_v2rhosigma_3 = V_ZERO;
        let mut acc_v2rhosigma_4 = V_ZERO;
        let mut acc_v2rhosigma_5 = V_ZERO;
        let mut acc_v2sigma2_0 = V_ZERO;
        let mut acc_v2sigma2_1 = V_ZERO;
        let mut acc_v2sigma2_2 = V_ZERO;
        let mut acc_v2sigma2_3 = V_ZERO;
        let mut acc_v2sigma2_4 = V_ZERO;
        let mut acc_v2sigma2_5 = V_ZERO;
        let mut acc_v3rho3_0 = V_ZERO;
        let mut acc_v3rho3_1 = V_ZERO;
        let mut acc_v3rho3_2 = V_ZERO;
        let mut acc_v3rho3_3 = V_ZERO;
        let mut acc_v3rho2sigma_0 = V_ZERO;
        let mut acc_v3rho2sigma_1 = V_ZERO;
        let mut acc_v3rho2sigma_2 = V_ZERO;
        let mut acc_v3rho2sigma_3 = V_ZERO;
        let mut acc_v3rho2sigma_4 = V_ZERO;
        let mut acc_v3rho2sigma_5 = V_ZERO;
        let mut acc_v3rho2sigma_6 = V_ZERO;
        let mut acc_v3rho2sigma_7 = V_ZERO;
        let mut acc_v3rho2sigma_8 = V_ZERO;
        let mut acc_v3rhosigma2_0 = V_ZERO;
        let mut acc_v3rhosigma2_1 = V_ZERO;
        let mut acc_v3rhosigma2_2 = V_ZERO;
        let mut acc_v3rhosigma2_3 = V_ZERO;
        let mut acc_v3rhosigma2_4 = V_ZERO;
        let mut acc_v3rhosigma2_5 = V_ZERO;
        let mut acc_v3rhosigma2_6 = V_ZERO;
        let mut acc_v3rhosigma2_7 = V_ZERO;
        let mut acc_v3rhosigma2_8 = V_ZERO;
        let mut acc_v3rhosigma2_9 = V_ZERO;
        let mut acc_v3rhosigma2_10 = V_ZERO;
        let mut acc_v3rhosigma2_11 = V_ZERO;
        let mut acc_v3sigma3_0 = V_ZERO;
        let mut acc_v3sigma3_1 = V_ZERO;
        let mut acc_v3sigma3_2 = V_ZERO;
        let mut acc_v3sigma3_3 = V_ZERO;
        let mut acc_v3sigma3_4 = V_ZERO;
        let mut acc_v3sigma3_5 = V_ZERO;
        let mut acc_v3sigma3_6 = V_ZERO;
        let mut acc_v3sigma3_7 = V_ZERO;
        let mut acc_v3sigma3_8 = V_ZERO;
        let mut acc_v3sigma3_9 = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = t2 * t2;
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 * t4 * f64x8::splat(M_PI);
            let t7 = v_rho0 + v_rho1;
            let t8 = f64x8::splat(1.0) / t7;
            let t11 = (f64x8::splat(2.0) * v_rho0 * t8).simd_le(zeta_threshold);
            let t12 = zeta_threshold - f64x8::splat(1.0);
            let t15 = (f64x8::splat(2.0) * v_rho1 * t8).simd_le(zeta_threshold);
            let t16 = -t12;
            let t17 = v_rho0 - v_rho1;
            let t19 = ((t11).select(t12, (t15).select(t16, t17 * t8)));
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * t22;
            let t24 = t23 * zeta_threshold;
            let t25 = (simd::cbrt(t20));
            let t26 = t25 * t25;
            let t28 = ((t21).select(t24, t26 * t20));
            let t29 = (simd::cbrt(t7));
            let t30 = t29 * t29;
            let t31 = t28 * t30;
            let t32 = f64x8::splat(1.0) / param_p;
            let t34 = f64x8::splat(M_CBRT6);
            let t35 = param_C2 * t32 * t34;
            let t36 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t37 = (simd::cbrt(t36));
            let t38 = t37 * t37;
            let t39 = f64x8::splat(1.0) / t38;
            let t41 = v_rho0 * v_rho0;
            let t42 = (simd::cbrt(v_rho0));
            let t43 = t42 * t42;
            let t45 = f64x8::splat(1.0) / t43 / t41;
            let t49 = f64x8::splat(1.0) + t35 * t39 * v_sigma0 * t45 / f64x8::splat(24.0);
            let t50 = (simd::pow(t49, -param_p));
            let t51 = t31 * t50;
            let t52 = t6 * t51;
            let t54 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t52));
            let t55 = (v_rho1).simd_le(dens_threshold);
            let t56 = -t17;
            let t58 = ((t15).select(t12, (t11).select(t16, t56 * t8)));
            let t59 = f64x8::splat(1.0) + t58;
            let t60 = (t59).simd_le(zeta_threshold);
            let t61 = (simd::cbrt(t59));
            let t62 = t61 * t61;
            let t64 = ((t60).select(t24, t62 * t59));
            let t65 = t64 * t30;
            let t67 = v_rho1 * v_rho1;
            let t68 = (simd::cbrt(v_rho1));
            let t69 = t68 * t68;
            let t71 = f64x8::splat(1.0) / t69 / t67;
            let t75 = f64x8::splat(1.0) + t35 * t39 * v_sigma2 * t71 / f64x8::splat(24.0);
            let t76 = (simd::pow(t75, -param_p));
            let t77 = t65 * t76;
            let t78 = t6 * t77;
            let t80 = ((t55).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t78));
            let tzk0 = t54 + t80;
            acc_zk = tzk0;
            let t81 = t7 * t7;
            let t82 = f64x8::splat(1.0) / t81;
            let t83 = t17 * t82;
            let t85 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t83)));
            let t88 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t85));
            let t89 = t88 * t30;
            let t90 = t89 * t50;
            let t91 = t6 * t90;
            let t93 = f64x8::splat(1.0) / t29;
            let t94 = t28 * t93;
            let t95 = t94 * t50;
            let t96 = t6 * t95;
            let t97 = t96 / f64x8::splat(10.0);
            let t98 = param_C2 * t34;
            let t99 = t98 * t39;
            let t100 = t41 * v_rho0;
            let t102 = f64x8::splat(1.0) / t43 / t100;
            let t104 = f64x8::splat(1.0) / t49;
            let t106 = t99 * v_sigma0 * t102 * t104;
            let t110 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t91 + t97 + t52 * t106 / f64x8::splat(60.0)));
            let t111 = t56 * t82;
            let t113 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t111)));
            let t116 = ((t60).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t62 * t113));
            let t117 = t116 * t30;
            let t118 = t117 * t76;
            let t119 = t6 * t118;
            let t121 = t64 * t93;
            let t122 = t121 * t76;
            let t123 = t6 * t122;
            let t124 = t123 / f64x8::splat(10.0);
            let t126 = ((t55).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t119 + t124));
            let tvrho0 = t54 + t80 + t7 * (t110 + t126);
            acc_vrho_0 = tvrho0;
            let t130 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t83)));
            let t133 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t130));
            let t134 = t133 * t30;
            let t135 = t134 * t50;
            let t136 = t6 * t135;
            let t139 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t136 + t97));
            let t141 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t111)));
            let t144 = ((t60).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t62 * t141));
            let t145 = t144 * t30;
            let t146 = t145 * t76;
            let t147 = t6 * t146;
            let t149 = t67 * v_rho1;
            let t151 = f64x8::splat(1.0) / t69 / t149;
            let t153 = f64x8::splat(1.0) / t75;
            let t155 = t99 * v_sigma2 * t151 * t153;
            let t159 = ((t55).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t147 + t124 + t78 * t155 / f64x8::splat(60.0)));
            let tvrho1 = t54 + t80 + t7 * (t139 + t159);
            acc_vrho_1 = tvrho1;
            let t164 = t98 * t39 * t45 * t104;
            let t167 = ((t1).select(f64x8::splat(0.0), -t52 * t164 / f64x8::splat(160.0)));
            let tvsigma0 = t7 * t167;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t170 = t98 * t39 * t71 * t153;
            let t173 = ((t55).select(f64x8::splat(0.0), -t78 * t170 / f64x8::splat(160.0)));
            let tvsigma2 = t7 * t173;
            acc_vsigma_2 = tvsigma2;
            let t176 = f64x8::splat(1.0) / t25;
            let t177 = t85 * t85;
            let t180 = t81 * t7;
            let t181 = f64x8::splat(1.0) / t180;
            let t182 = t17 * t181;
            let t185 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t82 + f64x8::splat(2.0) * t182)));
            let t189 = ((t21).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t176 * t177 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t185));
            let t192 = t6 * t189 * t30 * t50;
            let t196 = t6 * t88 * t93 * t50;
            let t201 = f64x8::splat(1.0) / t29 / t7;
            let t204 = t6 * t28 * t201 * t50;
            let t205 = t204 / f64x8::splat(30.0);
            let t206 = t96 * t106;
            let t208 = param_C2 * param_C2;
            let t209 = t34 * t34;
            let t210 = t208 * t209;
            let t212 = f64x8::splat(1.0) / t37 / t36;
            let t213 = t210 * t212;
            let t214 = v_sigma0 * v_sigma0;
            let t215 = t41 * t41;
            let t218 = f64x8::splat(1.0) / t42 / t215 / t100;
            let t220 = t49 * t49;
            let t221 = f64x8::splat(1.0) / t220;
            let t223 = t213 * t214 * t218 * t221;
            let t227 = f64x8::splat(1.0) / t43 / t215;
            let t230 = t99 * v_sigma0 * t227 * t104;
            let t233 = t6 * t28;
            let t234 = t30 * t50;
            let t235 = t234 * t208;
            let t236 = t233 * t235;
            let t237 = t209 * t212;
            let t238 = t237 * t214;
            let t240 = t218 * t221 * t32;
            let t241 = t238 * t240;
            let t245 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t192 + t196 / f64x8::splat(5.0) + t91 * t106 / f64x8::splat(30.0) - t205 + t206 / f64x8::splat(45.0) + t52 * t223 / f64x8::splat(540.0) - f64x8::splat(11.0) / f64x8::splat(180.0) * t52 * t230 + t236 * t241 / f64x8::splat(540.0)));
            let t246 = f64x8::splat(1.0) / t61;
            let t247 = t113 * t113;
            let t250 = t56 * t181;
            let t253 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), f64x8::splat(2.0) * t82 + f64x8::splat(2.0) * t250)));
            let t257 = ((t60).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t246 * t247 + f64x8::splat(5.0) / f64x8::splat(3.0) * t62 * t253));
            let t260 = t6 * t257 * t30 * t76;
            let t264 = t6 * t116 * t93 * t76;
            let t268 = t6 * t64 * t201 * t76;
            let t269 = t268 / f64x8::splat(30.0);
            let t271 = ((t55).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t260 + t264 / f64x8::splat(5.0) - t269));
            let tv2rho20 = f64x8::splat(2.0) * t110 + f64x8::splat(2.0) * t126 + t7 * (t245 + t271);
            acc_v2rho2_0 = tv2rho20;
            let t274 = t176 * t130;
            let t278 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), f64x8::splat(2.0) * t182)));
            let t282 = ((t21).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t274 * t85 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t278));
            let t285 = t6 * t282 * t30 * t50;
            let t289 = t6 * t133 * t93 * t50;
            let t296 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t285 + t289 / f64x8::splat(10.0) + t136 * t106 / f64x8::splat(60.0) + t196 / f64x8::splat(10.0) - t205 + t206 / f64x8::splat(90.0)));
            let t297 = t246 * t141;
            let t301 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), f64x8::splat(2.0) * t250)));
            let t305 = ((t60).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t297 * t113 + f64x8::splat(5.0) / f64x8::splat(3.0) * t62 * t301));
            let t308 = t6 * t305 * t30 * t76;
            let t312 = t6 * t144 * t93 * t76;
            let t317 = t123 * t155;
            let t320 = ((t55).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t308 + t312 / f64x8::splat(10.0) + t264 / f64x8::splat(10.0) - t269 + t119 * t155 / f64x8::splat(60.0) + t317 / f64x8::splat(90.0)));
            let tv2rho21 = t110 + t126 + t139 + t159 + t7 * (t296 + t320);
            acc_v2rho2_1 = tv2rho21;
            let t325 = t130 * t130;
            let t330 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), f64x8::splat(2.0) * t82 + f64x8::splat(2.0) * t182)));
            let t334 = ((t21).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t176 * t325 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t330));
            let t337 = t6 * t334 * t30 * t50;
            let t341 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t337 + t289 / f64x8::splat(5.0) - t205));
            let t342 = t141 * t141;
            let t347 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t82 + f64x8::splat(2.0) * t250)));
            let t351 = ((t60).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t246 * t342 + f64x8::splat(5.0) / f64x8::splat(3.0) * t62 * t347));
            let t354 = t6 * t351 * t30 * t76;
            let t360 = v_sigma2 * v_sigma2;
            let t361 = t67 * t67;
            let t364 = f64x8::splat(1.0) / t68 / t361 / t149;
            let t366 = t75 * t75;
            let t367 = f64x8::splat(1.0) / t366;
            let t369 = t213 * t360 * t364 * t367;
            let t373 = f64x8::splat(1.0) / t69 / t361;
            let t376 = t99 * v_sigma2 * t373 * t153;
            let t379 = t6 * t64;
            let t380 = t30 * t76;
            let t381 = t380 * t208;
            let t382 = t379 * t381;
            let t383 = t237 * t360;
            let t385 = t364 * t367 * t32;
            let t386 = t383 * t385;
            let t390 = ((t55).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t354 + t312 / f64x8::splat(5.0) + t147 * t155 / f64x8::splat(30.0) - t269 + t317 / f64x8::splat(45.0) + t78 * t369 / f64x8::splat(540.0) - f64x8::splat(11.0) / f64x8::splat(180.0) * t78 * t376 + t382 * t386 / f64x8::splat(540.0)));
            let tv2rho22 = f64x8::splat(2.0) * t139 + f64x8::splat(2.0) * t159 + t7 * (t341 + t390);
            acc_v2rho2_2 = tv2rho22;
            let t396 = t96 * t164 / f64x8::splat(240.0);
            let t397 = t215 * t41;
            let t399 = f64x8::splat(1.0) / t42 / t397;
            let t402 = t213 * v_sigma0 * t399 * t221;
            let t407 = t98 * t39 * t102 * t104;
            let t412 = t221 * t32 * v_sigma0;
            let t413 = t237 * t399 * t412;
            let t417 = ((t1).select(f64x8::splat(0.0), -t91 * t164 / f64x8::splat(160.0) - t396 - t52 * t402 / f64x8::splat(1440.0) + t52 * t407 / f64x8::splat(60.0) - t236 * t413 / f64x8::splat(1440.0)));
            let tv2rhosigma0 = t7 * t417 + t167;
            acc_v2rhosigma_0 = tv2rhosigma0;
            let tv2rhosigma1 = f64x8::splat(0.0);
            acc_v2rhosigma_1 = tv2rhosigma1;
            let t422 = t123 * t170 / f64x8::splat(240.0);
            let t424 = ((t55).select(f64x8::splat(0.0), -t119 * t170 / f64x8::splat(160.0) - t422));
            let tv2rhosigma2 = t7 * t424 + t173;
            acc_v2rhosigma_2 = tv2rhosigma2;
            let t429 = ((t1).select(f64x8::splat(0.0), -t136 * t164 / f64x8::splat(160.0) - t396));
            let tv2rhosigma3 = t7 * t429 + t167;
            acc_v2rhosigma_3 = tv2rhosigma3;
            let tv2rhosigma4 = f64x8::splat(0.0);
            acc_v2rhosigma_4 = tv2rhosigma4;
            let t433 = t361 * t67;
            let t435 = f64x8::splat(1.0) / t68 / t433;
            let t438 = t213 * v_sigma2 * t435 * t367;
            let t443 = t98 * t39 * t151 * t153;
            let t448 = t367 * t32 * v_sigma2;
            let t449 = t237 * t435 * t448;
            let t453 = ((t55).select(f64x8::splat(0.0), -t147 * t170 / f64x8::splat(160.0) - t422 - t78 * t438 / f64x8::splat(1440.0) + t78 * t443 / f64x8::splat(60.0) - t382 * t449 / f64x8::splat(1440.0)));
            let tv2rhosigma5 = t7 * t453 + t173;
            acc_v2rhosigma_5 = tv2rhosigma5;
            let t455 = t215 * v_rho0;
            let t457 = f64x8::splat(1.0) / t42 / t455;
            let t460 = t210 * t212 * t457 * t221;
            let t464 = t213 * t457 * t221 * t32;
            let t468 = ((t1).select(f64x8::splat(0.0), t52 * t460 / f64x8::splat(3840.0) + t52 * t464 / f64x8::splat(3840.0)));
            let tv2sigma20 = t7 * t468;
            acc_v2sigma2_0 = tv2sigma20;
            let tv2sigma21 = f64x8::splat(0.0);
            acc_v2sigma2_1 = tv2sigma21;
            let tv2sigma22 = f64x8::splat(0.0);
            acc_v2sigma2_2 = tv2sigma22;
            let tv2sigma23 = f64x8::splat(0.0);
            acc_v2sigma2_3 = tv2sigma23;
            let tv2sigma24 = f64x8::splat(0.0);
            acc_v2sigma2_4 = tv2sigma24;
            let t469 = t361 * v_rho1;
            let t471 = f64x8::splat(1.0) / t68 / t469;
            let t474 = t210 * t212 * t471 * t367;
            let t478 = t213 * t471 * t367 * t32;
            let t482 = ((t55).select(f64x8::splat(0.0), t78 * t474 / f64x8::splat(3840.0) + t78 * t478 / f64x8::splat(3840.0)));
            let tv2sigma25 = t7 * t482;
            acc_v2sigma2_5 = tv2sigma25;
            let t486 = f64x8::splat(1.0) / t25 / t20;
            let t487 = t177 * t85;
            let t490 = t176 * t85;
            let t493 = t81 * t81;
            let t494 = f64x8::splat(1.0) / t493;
            let t495 = t17 * t494;
            let t498 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), f64x8::splat(6.0) * t181 - f64x8::splat(6.0) * t495)));
            let t502 = ((t21).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t486 * t487 + f64x8::splat(10.0) / f64x8::splat(3.0) * t490 * t185 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t498));
            let t505 = t6 * t502 * t30 * t50;
            let t507 = t215 * t215;
            let t509 = f64x8::splat(1.0) / t42 / t507;
            let t512 = t213 * t214 * t509 * t221;
            let t516 = f64x8::splat(1.0) / t43 / t455;
            let t519 = t99 * v_sigma0 * t516 * t104;
            let t524 = t238 * t509 * t221 * t32;
            let t529 = t6 * t88;
            let t530 = t529 * t235;
            let t533 = t96 * t230;
            let t536 = t93 * t50 * t208;
            let t537 = t233 * t536;
            let t538 = t537 * t241;
            let t540 = t4 * t4;
            let t543 = t3 / t540 / t36;
            let t544 = t543 * t51;
            let t545 = t208 * param_C2;
            let t546 = t214 * v_sigma0;
            let t547 = t545 * t546;
            let t548 = t507 * t100;
            let t549 = f64x8::splat(1.0) / t548;
            let t551 = f64x8::splat(1.0) / t220 / t49;
            let t552 = t549 * t551;
            let t553 = t552 * t32;
            let t554 = t547 * t553;
            let t557 = param_p * param_p;
            let t558 = f64x8::splat(1.0) / t557;
            let t560 = t547 * t552 * t558;
            let t563 = t196 * t106;
            let t567 = t204 * t106;
            let t569 = t96 * t223;
            let t575 = t6 * t189 * t93 * t50;
            let t579 = t6 * t88 * t201 * t50;
            let t582 = f64x8::splat(1.0) / t29 / t81;
            let t585 = t6 * t28 * t582 * t50;
            let t586 = f64x8::splat(2.0) / f64x8::splat(45.0) * t585;
            let t587 = t543 * t31;
            let t588 = t50 * t545;
            let t591 = t588 * t546 * t549 * t551;
            let t594 = f64x8::splat(3.0) / f64x8::splat(20.0) * t505 - f64x8::splat(11.0) / f64x8::splat(540.0) * t52 * t512 + f64x8::splat(77.0) / f64x8::splat(270.0) * t52 * t519 - f64x8::splat(11.0) / f64x8::splat(540.0) * t236 * t524 - f64x8::splat(11.0) / f64x8::splat(60.0) * t91 * t230 + t530 * t241 / f64x8::splat(180.0) - f64x8::splat(11.0) / f64x8::splat(90.0) * t533 + t538 / f64x8::splat(270.0) + t544 * t554 / f64x8::splat(270.0) + t544 * t560 / f64x8::splat(405.0) + t563 / f64x8::splat(15.0) + t91 * t223 / f64x8::splat(180.0) - t567 / f64x8::splat(90.0) + t569 / f64x8::splat(270.0) + t192 * t106 / f64x8::splat(20.0) + f64x8::splat(3.0) / f64x8::splat(10.0) * t575 - t579 / f64x8::splat(10.0) + t586 + t587 * t591 / f64x8::splat(810.0);
            let t595 = ((t1).select(f64x8::splat(0.0), t594));
            let t597 = f64x8::splat(1.0) / t61 / t59;
            let t598 = t247 * t113;
            let t601 = t246 * t113;
            let t604 = t56 * t494;
            let t607 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t181 - f64x8::splat(6.0) * t604)));
            let t611 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t597 * t598 + f64x8::splat(10.0) / f64x8::splat(3.0) * t601 * t253 + f64x8::splat(5.0) / f64x8::splat(3.0) * t62 * t607));
            let t614 = t6 * t611 * t30 * t76;
            let t618 = t6 * t257 * t93 * t76;
            let t622 = t6 * t116 * t201 * t76;
            let t626 = t6 * t64 * t582 * t76;
            let t627 = f64x8::splat(2.0) / f64x8::splat(45.0) * t626;
            let t629 = ((t55).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t614 + f64x8::splat(3.0) / f64x8::splat(10.0) * t618 - t622 / f64x8::splat(10.0) + t627));
            let tv3rho30 = f64x8::splat(3.0) * t245 + f64x8::splat(3.0) * t271 + t7 * (t595 + t629);
            acc_v3rho3_0 = tv3rho30;
            let t632 = f64x8::splat(2.0) * t296;
            let t633 = f64x8::splat(2.0) * t320;
            let t634 = t486 * t130;
            let t637 = t176 * t278;
            let t642 = f64x8::splat(2.0) * t181;
            let t643 = f64x8::splat(6.0) * t495;
            let t645 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t642 - t643)));
            let t649 = ((t21).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t634 * t177 + f64x8::splat(20.0) / f64x8::splat(9.0) * t637 * t85 + f64x8::splat(10.0) / f64x8::splat(9.0) * t274 * t185 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t645));
            let t652 = t6 * t649 * t30 * t50;
            let t656 = t6 * t282 * t93 * t50;
            let t657 = t656 / f64x8::splat(5.0);
            let t662 = t6 * t133 * t201 * t50;
            let t665 = t289 * t106 / f64x8::splat(45.0);
            let t670 = t6 * t133;
            let t671 = t670 * t235;
            let t681 = f64x8::splat(3.0) / f64x8::splat(20.0) * t652 + t657 + t285 * t106 / f64x8::splat(30.0) - t662 / f64x8::splat(30.0) + t665 + t136 * t223 / f64x8::splat(540.0) - f64x8::splat(11.0) / f64x8::splat(180.0) * t136 * t230 + t671 * t241 / f64x8::splat(540.0) + t575 / f64x8::splat(10.0) - t579 / f64x8::splat(15.0) + t563 / f64x8::splat(45.0) + t586 - t567 / f64x8::splat(135.0) + t569 / f64x8::splat(810.0) - f64x8::splat(11.0) / f64x8::splat(270.0) * t533 + t538 / f64x8::splat(810.0);
            let t682 = ((t1).select(f64x8::splat(0.0), t681));
            let t683 = t597 * t141;
            let t686 = t246 * t301;
            let t691 = f64x8::splat(6.0) * t604;
            let t693 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t642 - t691)));
            let t697 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t683 * t247 + f64x8::splat(20.0) / f64x8::splat(9.0) * t686 * t113 + f64x8::splat(10.0) / f64x8::splat(9.0) * t297 * t253 + f64x8::splat(5.0) / f64x8::splat(3.0) * t62 * t693));
            let t700 = t6 * t697 * t30 * t76;
            let t704 = t6 * t305 * t93 * t76;
            let t705 = t704 / f64x8::splat(5.0);
            let t708 = t6 * t144 * t201 * t76;
            let t715 = t264 * t155 / f64x8::splat(45.0);
            let t716 = t268 * t155;
            let t719 = ((t55).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t700 + t705 - t708 / f64x8::splat(30.0) + t618 / f64x8::splat(10.0) - t622 / f64x8::splat(15.0) + t627 + t260 * t155 / f64x8::splat(60.0) + t715 - t716 / f64x8::splat(270.0)));
            let tv3rho31 = t245 + t271 + t632 + t633 + t7 * (t682 + t719);
            acc_v3rho3_1 = tv3rho31;
            let t722 = t486 * t325;
            let t727 = t176 * t330;
            let t731 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t642 - t643)));
            let t735 = ((t21).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t722 * t85 + f64x8::splat(20.0) / f64x8::splat(9.0) * t274 * t278 + f64x8::splat(10.0) / f64x8::splat(9.0) * t727 * t85 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t731));
            let t738 = t6 * t735 * t30 * t50;
            let t742 = t6 * t334 * t93 * t50;
            let t750 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t738 + t742 / f64x8::splat(10.0) + t337 * t106 / f64x8::splat(60.0) + t657 - t662 / f64x8::splat(15.0) + t665 - t579 / f64x8::splat(30.0) + t586 - t567 / f64x8::splat(270.0)));
            let t751 = t597 * t342;
            let t756 = t246 * t347;
            let t760 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t642 - t691)));
            let t764 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t751 * t113 + f64x8::splat(20.0) / f64x8::splat(9.0) * t297 * t301 + f64x8::splat(10.0) / f64x8::splat(9.0) * t756 * t113 + f64x8::splat(5.0) / f64x8::splat(3.0) * t62 * t760));
            let t767 = t6 * t764 * t30 * t76;
            let t771 = t6 * t351 * t93 * t76;
            let t776 = t312 * t155;
            let t782 = t123 * t369;
            let t786 = t123 * t376;
            let t788 = t6 * t116;
            let t789 = t788 * t381;
            let t793 = t93 * t76 * t208;
            let t794 = t379 * t793;
            let t795 = t794 * t386;
            let t797 = f64x8::splat(3.0) / f64x8::splat(20.0) * t767 + t771 / f64x8::splat(10.0) + t705 - t708 / f64x8::splat(15.0) + t308 * t155 / f64x8::splat(30.0) + t776 / f64x8::splat(45.0) - t622 / f64x8::splat(30.0) + t627 + t715 - t716 / f64x8::splat(135.0) + t119 * t369 / f64x8::splat(540.0) + t782 / f64x8::splat(810.0) - f64x8::splat(11.0) / f64x8::splat(180.0) * t119 * t376 - f64x8::splat(11.0) / f64x8::splat(270.0) * t786 + t789 * t386 / f64x8::splat(540.0) + t795 / f64x8::splat(810.0);
            let t798 = ((t55).select(f64x8::splat(0.0), t797));
            let tv3rho32 = t632 + t633 + t341 + t390 + t7 * (t750 + t798);
            acc_v3rho3_2 = tv3rho32;
            let t803 = t325 * t130;
            let t810 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t181 - f64x8::splat(6.0) * t495)));
            let t814 = ((t21).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t486 * t803 + f64x8::splat(10.0) / f64x8::splat(3.0) * t274 * t330 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t810));
            let t817 = t6 * t814 * t30 * t50;
            let t822 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t817 + f64x8::splat(3.0) / f64x8::splat(10.0) * t742 - t662 / f64x8::splat(10.0) + t586));
            let t823 = t342 * t141;
            let t830 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), f64x8::splat(6.0) * t181 - f64x8::splat(6.0) * t604)));
            let t834 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t597 * t823 + f64x8::splat(10.0) / f64x8::splat(3.0) * t297 * t347 + f64x8::splat(5.0) / f64x8::splat(3.0) * t62 * t830));
            let t837 = t6 * t834 * t30 * t76;
            let t840 = t361 * t361;
            let t842 = f64x8::splat(1.0) / t68 / t840;
            let t845 = t213 * t360 * t842 * t367;
            let t849 = f64x8::splat(1.0) / t69 / t469;
            let t852 = t99 * v_sigma2 * t849 * t153;
            let t857 = t383 * t842 * t367 * t32;
            let t862 = t6 * t144;
            let t863 = t862 * t381;
            let t870 = t543 * t77;
            let t871 = t360 * v_sigma2;
            let t872 = t545 * t871;
            let t873 = t840 * t149;
            let t874 = f64x8::splat(1.0) / t873;
            let t876 = f64x8::splat(1.0) / t366 / t75;
            let t877 = t874 * t876;
            let t878 = t877 * t32;
            let t879 = t872 * t878;
            let t883 = t872 * t877 * t558;
            let t892 = t543 * t65;
            let t893 = t76 * t545;
            let t896 = t893 * t871 * t874 * t876;
            let t899 = f64x8::splat(3.0) / f64x8::splat(20.0) * t837 + t795 / f64x8::splat(270.0) - f64x8::splat(11.0) / f64x8::splat(540.0) * t78 * t845 + f64x8::splat(77.0) / f64x8::splat(270.0) * t78 * t852 - f64x8::splat(11.0) / f64x8::splat(540.0) * t382 * t857 - f64x8::splat(11.0) / f64x8::splat(60.0) * t147 * t376 + t863 * t386 / f64x8::splat(180.0) - f64x8::splat(11.0) / f64x8::splat(90.0) * t786 - t716 / f64x8::splat(90.0) + t776 / f64x8::splat(15.0) + t782 / f64x8::splat(270.0) + t870 * t879 / f64x8::splat(270.0) + t870 * t883 / f64x8::splat(405.0) + t354 * t155 / f64x8::splat(20.0) + t147 * t369 / f64x8::splat(180.0) + f64x8::splat(3.0) / f64x8::splat(10.0) * t771 + t627 - t708 / f64x8::splat(10.0) + t892 * t896 / f64x8::splat(810.0);
            let t900 = ((t55).select(f64x8::splat(0.0), t899));
            let tv3rho33 = f64x8::splat(3.0) * t341 + f64x8::splat(3.0) * t390 + t7 * (t822 + t900);
            acc_v3rho3_3 = tv3rho33;
            let t905 = t237 * t218 * t412;
            let t910 = t537 * t413;
            let t914 = t213 * v_sigma0 * t218 * t221;
            let t917 = t507 * t41;
            let t918 = f64x8::splat(1.0) / t917;
            let t919 = t545 * t918;
            let t920 = t551 * t558;
            let t921 = t920 * t214;
            let t922 = t919 * t921;
            let t927 = t545 * t214;
            let t930 = t927 * t918 * t551 * t32;
            let t934 = t204 * t164 / f64x8::splat(720.0);
            let t935 = t96 * t407;
            let t937 = t96 * t402;
            let t941 = t98 * t39 * t227 * t104;
            let t944 = t196 * t164;
            let t952 = t588 * t214 * t918 * t551;
            let t955 = t236 * t905 / f64x8::splat(160.0) - t530 * t413 / f64x8::splat(720.0) - t910 / f64x8::splat(1080.0) + t52 * t914 / f64x8::splat(160.0) - t544 * t922 / f64x8::splat(1080.0) - t192 * t164 / f64x8::splat(160.0) - t544 * t930 / f64x8::splat(720.0) + t934 + t935 / f64x8::splat(45.0) - t937 / f64x8::splat(1080.0) - f64x8::splat(11.0) / f64x8::splat(180.0) * t52 * t941 - t944 / f64x8::splat(120.0) + t91 * t407 / f64x8::splat(30.0) - t91 * t402 / f64x8::splat(720.0) - t587 * t952 / f64x8::splat(2160.0);
            let t956 = ((t1).select(f64x8::splat(0.0), t955));
            let tv3rho2sigma0 = t7 * t956 + f64x8::splat(2.0) * t417;
            acc_v3rho2sigma_0 = tv3rho2sigma0;
            let tv3rho2sigma1 = f64x8::splat(0.0);
            acc_v3rho2sigma_1 = tv3rho2sigma1;
            let t961 = t264 * t170;
            let t964 = t268 * t170 / f64x8::splat(720.0);
            let t966 = ((t55).select(f64x8::splat(0.0), -t260 * t170 / f64x8::splat(160.0) - t961 / f64x8::splat(120.0) + t964));
            let tv3rho2sigma2 = t7 * t966 + f64x8::splat(2.0) * t424;
            acc_v3rho2sigma_2 = tv3rho2sigma2;
            let t970 = t289 * t164;
            let t983 = ((t1).select(f64x8::splat(0.0), -t285 * t164 / f64x8::splat(160.0) - t970 / f64x8::splat(240.0) - t136 * t402 / f64x8::splat(1440.0) + t136 * t407 / f64x8::splat(60.0) - t671 * t413 / f64x8::splat(1440.0) - t944 / f64x8::splat(240.0) + t934 - t937 / f64x8::splat(2160.0) + t935 / f64x8::splat(90.0) - t910 / f64x8::splat(2160.0)));
            let tv3rho2sigma3 = t7 * t983 + t417 + t429;
            acc_v3rho2sigma_3 = tv3rho2sigma3;
            let tv3rho2sigma4 = f64x8::splat(0.0);
            acc_v3rho2sigma_4 = tv3rho2sigma4;
            let t987 = t312 * t170;
            let t992 = t123 * t438;
            let t996 = t123 * t443;
            let t1000 = t794 * t449;
            let t1003 = ((t55).select(f64x8::splat(0.0), -t308 * t170 / f64x8::splat(160.0) - t987 / f64x8::splat(240.0) - t961 / f64x8::splat(240.0) + t964 - t119 * t438 / f64x8::splat(1440.0) - t992 / f64x8::splat(2160.0) + t119 * t443 / f64x8::splat(60.0) + t996 / f64x8::splat(90.0) - t789 * t449 / f64x8::splat(1440.0) - t1000 / f64x8::splat(2160.0)));
            let tv3rho2sigma5 = t7 * t1003 + t424 + t453;
            acc_v3rho2sigma_5 = tv3rho2sigma5;
            let t1010 = ((t1).select(f64x8::splat(0.0), -t337 * t164 / f64x8::splat(160.0) - t970 / f64x8::splat(120.0) + t934));
            let tv3rho2sigma6 = t7 * t1010 + f64x8::splat(2.0) * t429;
            acc_v3rho2sigma_6 = tv3rho2sigma6;
            let tv3rho2sigma7 = f64x8::splat(0.0);
            acc_v3rho2sigma_7 = tv3rho2sigma7;
            let t1016 = t213 * v_sigma2 * t364 * t367;
            let t1020 = t237 * t364 * t448;
            let t1028 = t545 * t360;
            let t1029 = t840 * t67;
            let t1030 = f64x8::splat(1.0) / t1029;
            let t1033 = t1028 * t1030 * t876 * t32;
            let t1036 = t545 * t1030;
            let t1037 = t876 * t558;
            let t1038 = t1037 * t360;
            let t1039 = t1036 * t1038;
            let t1044 = t98 * t39 * t373 * t153;
            let t1055 = t893 * t360 * t1030 * t876;
            let t1058 = -t1000 / f64x8::splat(1080.0) + t78 * t1016 / f64x8::splat(160.0) + t382 * t1020 / f64x8::splat(160.0) - t863 * t449 / f64x8::splat(720.0) - t992 / f64x8::splat(1080.0) - t354 * t170 / f64x8::splat(160.0) - t870 * t1033 / f64x8::splat(720.0) - t870 * t1039 / f64x8::splat(1080.0) - f64x8::splat(11.0) / f64x8::splat(180.0) * t78 * t1044 + t147 * t443 / f64x8::splat(30.0) - t147 * t438 / f64x8::splat(720.0) - t987 / f64x8::splat(120.0) + t996 / f64x8::splat(45.0) + t964 - t892 * t1055 / f64x8::splat(2160.0);
            let t1059 = ((t55).select(f64x8::splat(0.0), t1058));
            let tv3rho2sigma8 = t7 * t1059 + f64x8::splat(2.0) * t453;
            acc_v3rho2sigma_8 = tv3rho2sigma8;
            let t1064 = t96 * t460 / f64x8::splat(5760.0);
            let t1065 = t507 * v_rho0;
            let t1066 = f64x8::splat(1.0) / t1065;
            let t1069 = t588 * v_sigma0 * t1066 * t551;
            let t1074 = t210 * t212 * t399 * t221;
            let t1077 = t545 * t1066;
            let t1079 = t551 * t32 * v_sigma0;
            let t1080 = t1077 * t1079;
            let t1086 = t96 * t464 / f64x8::splat(5760.0);
            let t1089 = t213 * t399 * t221 * t32;
            let t1092 = t920 * v_sigma0;
            let t1093 = t1077 * t1092;
            let t1097 = ((t1).select(f64x8::splat(0.0), t91 * t460 / f64x8::splat(3840.0) + t1064 + t587 * t1069 / f64x8::splat(5760.0) - t52 * t1074 / f64x8::splat(720.0) + t544 * t1080 / f64x8::splat(1920.0) + t91 * t464 / f64x8::splat(3840.0) + t1086 - t52 * t1089 / f64x8::splat(720.0) + t544 * t1093 / f64x8::splat(2880.0)));
            let tv3rhosigma20 = t7 * t1097 + t468;
            acc_v3rhosigma2_0 = tv3rhosigma20;
            let tv3rhosigma21 = f64x8::splat(0.0);
            acc_v3rhosigma2_1 = tv3rhosigma21;
            let tv3rhosigma22 = f64x8::splat(0.0);
            acc_v3rhosigma2_2 = tv3rhosigma22;
            let tv3rhosigma23 = f64x8::splat(0.0);
            acc_v3rhosigma2_3 = tv3rhosigma23;
            let tv3rhosigma24 = f64x8::splat(0.0);
            acc_v3rhosigma2_4 = tv3rhosigma24;
            let t1102 = t123 * t474 / f64x8::splat(5760.0);
            let t1106 = t123 * t478 / f64x8::splat(5760.0);
            let t1108 = ((t55).select(f64x8::splat(0.0), t119 * t474 / f64x8::splat(3840.0) + t1102 + t119 * t478 / f64x8::splat(3840.0) + t1106));
            let tv3rhosigma25 = t7 * t1108 + t482;
            acc_v3rhosigma2_5 = tv3rhosigma25;
            let t1115 = ((t1).select(f64x8::splat(0.0), t136 * t460 / f64x8::splat(3840.0) + t1064 + t136 * t464 / f64x8::splat(3840.0) + t1086));
            let tv3rhosigma26 = t7 * t1115 + t468;
            acc_v3rhosigma2_6 = tv3rhosigma26;
            let tv3rhosigma27 = f64x8::splat(0.0);
            acc_v3rhosigma2_7 = tv3rhosigma27;
            let tv3rhosigma28 = f64x8::splat(0.0);
            acc_v3rhosigma2_8 = tv3rhosigma28;
            let tv3rhosigma29 = f64x8::splat(0.0);
            acc_v3rhosigma2_9 = tv3rhosigma29;
            let tv3rhosigma210 = f64x8::splat(0.0);
            acc_v3rhosigma2_10 = tv3rhosigma210;
            let t1119 = t840 * v_rho1;
            let t1120 = f64x8::splat(1.0) / t1119;
            let t1123 = t893 * v_sigma2 * t1120 * t876;
            let t1128 = t210 * t212 * t435 * t367;
            let t1131 = t545 * t1120;
            let t1133 = t876 * t32 * v_sigma2;
            let t1134 = t1131 * t1133;
            let t1141 = t213 * t435 * t367 * t32;
            let t1144 = t1037 * v_sigma2;
            let t1145 = t1131 * t1144;
            let t1149 = ((t55).select(f64x8::splat(0.0), t147 * t474 / f64x8::splat(3840.0) + t1102 + t892 * t1123 / f64x8::splat(5760.0) - t78 * t1128 / f64x8::splat(720.0) + t870 * t1134 / f64x8::splat(1920.0) + t147 * t478 / f64x8::splat(3840.0) + t1106 - t78 * t1141 / f64x8::splat(720.0) + t870 * t1145 / f64x8::splat(2880.0)));
            let tv3rhosigma211 = t7 * t1149 + t482;
            acc_v3rhosigma2_11 = tv3rhosigma211;
            let t1152 = f64x8::splat(1.0) / t507 * t551;
            let t1153 = t588 * t1152;
            let t1157 = t588 * t1152 * t32;
            let t1161 = t588 * t1152 * t558;
            let t1165 = ((t1).select(f64x8::splat(0.0), -t587 * t1153 / f64x8::splat(15360.0) - t587 * t1157 / f64x8::splat(5120.0) - t587 * t1161 / f64x8::splat(7680.0)));
            let tv3sigma30 = t7 * t1165;
            acc_v3sigma3_0 = tv3sigma30;
            let tv3sigma31 = f64x8::splat(0.0);
            acc_v3sigma3_1 = tv3sigma31;
            let tv3sigma32 = f64x8::splat(0.0);
            acc_v3sigma3_2 = tv3sigma32;
            let tv3sigma33 = f64x8::splat(0.0);
            acc_v3sigma3_3 = tv3sigma33;
            let tv3sigma34 = f64x8::splat(0.0);
            acc_v3sigma3_4 = tv3sigma34;
            let tv3sigma35 = f64x8::splat(0.0);
            acc_v3sigma3_5 = tv3sigma35;
            let tv3sigma36 = f64x8::splat(0.0);
            acc_v3sigma3_6 = tv3sigma36;
            let tv3sigma37 = f64x8::splat(0.0);
            acc_v3sigma3_7 = tv3sigma37;
            let tv3sigma38 = f64x8::splat(0.0);
            acc_v3sigma3_8 = tv3sigma38;
            let t1167 = f64x8::splat(1.0) / t840 * t876;
            let t1168 = t893 * t1167;
            let t1172 = t893 * t1167 * t32;
            let t1176 = t893 * t1167 * t558;
            let t1180 = ((t55).select(f64x8::splat(0.0), -t892 * t1168 / f64x8::splat(15360.0) - t892 * t1172 / f64x8::splat(5120.0) - t892 * t1176 / f64x8::splat(7680.0)));
            let tv3sigma39 = t7 * t1180;
            acc_v3sigma3_9 = tv3sigma39;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(v2rho2, ip, m, 3, 0, acc_v2rho2_0);
        store_strided(v2rho2, ip, m, 3, 1, acc_v2rho2_1);
        store_strided(v2rho2, ip, m, 3, 2, acc_v2rho2_2);
        store_strided(v2rhosigma, ip, m, 6, 0, acc_v2rhosigma_0);
        store_strided(v2rhosigma, ip, m, 6, 1, acc_v2rhosigma_1);
        store_strided(v2rhosigma, ip, m, 6, 2, acc_v2rhosigma_2);
        store_strided(v2rhosigma, ip, m, 6, 3, acc_v2rhosigma_3);
        store_strided(v2rhosigma, ip, m, 6, 4, acc_v2rhosigma_4);
        store_strided(v2rhosigma, ip, m, 6, 5, acc_v2rhosigma_5);
        store_strided(v2sigma2, ip, m, 6, 0, acc_v2sigma2_0);
        store_strided(v2sigma2, ip, m, 6, 1, acc_v2sigma2_1);
        store_strided(v2sigma2, ip, m, 6, 2, acc_v2sigma2_2);
        store_strided(v2sigma2, ip, m, 6, 3, acc_v2sigma2_3);
        store_strided(v2sigma2, ip, m, 6, 4, acc_v2sigma2_4);
        store_strided(v2sigma2, ip, m, 6, 5, acc_v2sigma2_5);
        store_strided(v3rho3, ip, m, 4, 0, acc_v3rho3_0);
        store_strided(v3rho3, ip, m, 4, 1, acc_v3rho3_1);
        store_strided(v3rho3, ip, m, 4, 2, acc_v3rho3_2);
        store_strided(v3rho3, ip, m, 4, 3, acc_v3rho3_3);
        store_strided(v3rho2sigma, ip, m, 9, 0, acc_v3rho2sigma_0);
        store_strided(v3rho2sigma, ip, m, 9, 1, acc_v3rho2sigma_1);
        store_strided(v3rho2sigma, ip, m, 9, 2, acc_v3rho2sigma_2);
        store_strided(v3rho2sigma, ip, m, 9, 3, acc_v3rho2sigma_3);
        store_strided(v3rho2sigma, ip, m, 9, 4, acc_v3rho2sigma_4);
        store_strided(v3rho2sigma, ip, m, 9, 5, acc_v3rho2sigma_5);
        store_strided(v3rho2sigma, ip, m, 9, 6, acc_v3rho2sigma_6);
        store_strided(v3rho2sigma, ip, m, 9, 7, acc_v3rho2sigma_7);
        store_strided(v3rho2sigma, ip, m, 9, 8, acc_v3rho2sigma_8);
        store_strided(v3rhosigma2, ip, m, 12, 0, acc_v3rhosigma2_0);
        store_strided(v3rhosigma2, ip, m, 12, 1, acc_v3rhosigma2_1);
        store_strided(v3rhosigma2, ip, m, 12, 2, acc_v3rhosigma2_2);
        store_strided(v3rhosigma2, ip, m, 12, 3, acc_v3rhosigma2_3);
        store_strided(v3rhosigma2, ip, m, 12, 4, acc_v3rhosigma2_4);
        store_strided(v3rhosigma2, ip, m, 12, 5, acc_v3rhosigma2_5);
        store_strided(v3rhosigma2, ip, m, 12, 6, acc_v3rhosigma2_6);
        store_strided(v3rhosigma2, ip, m, 12, 7, acc_v3rhosigma2_7);
        store_strided(v3rhosigma2, ip, m, 12, 8, acc_v3rhosigma2_8);
        store_strided(v3rhosigma2, ip, m, 12, 9, acc_v3rhosigma2_9);
        store_strided(v3rhosigma2, ip, m, 12, 10, acc_v3rhosigma2_10);
        store_strided(v3rhosigma2, ip, m, 12, 11, acc_v3rhosigma2_11);
        store_strided(v3sigma3, ip, m, 10, 0, acc_v3sigma3_0);
        store_strided(v3sigma3, ip, m, 10, 1, acc_v3sigma3_1);
        store_strided(v3sigma3, ip, m, 10, 2, acc_v3sigma3_2);
        store_strided(v3sigma3, ip, m, 10, 3, acc_v3sigma3_3);
        store_strided(v3sigma3, ip, m, 10, 4, acc_v3sigma3_4);
        store_strided(v3sigma3, ip, m, 10, 5, acc_v3sigma3_5);
        store_strided(v3sigma3, ip, m, 10, 6, acc_v3sigma3_6);
        store_strided(v3sigma3, ip, m, 10, 7, acc_v3sigma3_7);
        store_strided(v3sigma3, ip, m, 10, 8, acc_v3sigma3_8);
        store_strided(v3sigma3, ip, m, 10, 9, acc_v3sigma3_9);
        ip += 8;
    }
}
