//! GGA_X_EV93 kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ev93.c`
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
pub fn gga_x_ev93_kxc_pol(
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
    param_a1: f64,
    param_a2: f64,
    param_a3: f64,
    param_b1: f64,
    param_b2: f64,
    param_b3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a1 = f64x8::splat(param_a1);
    let param_a2 = f64x8::splat(param_a2);
    let param_a3 = f64x8::splat(param_a3);
    let param_b1 = f64x8::splat(param_b1);
    let param_b2 = f64x8::splat(param_b2);
    let param_b3 = f64x8::splat(param_b3);
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
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t16 * t7)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = t5 * t25;
            let t27 = (simd::cbrt(t6));
            let t28 = f64x8::splat(M_CBRT6);
            let t29 = param_a1 * t28;
            let t30 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = t31 * t31;
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = t33 * v_sigma0;
            let t35 = v_rho0 * v_rho0;
            let t36 = (simd::cbrt(v_rho0));
            let t37 = t36 * t36;
            let t39 = f64x8::splat(1.0) / t37 / t35;
            let t40 = t34 * t39;
            let t43 = t28 * t28;
            let t44 = param_a2 * t43;
            let t46 = f64x8::splat(1.0) / t31 / t30;
            let t47 = v_sigma0 * v_sigma0;
            let t48 = t46 * t47;
            let t49 = t35 * t35;
            let t50 = t49 * v_rho0;
            let t52 = f64x8::splat(1.0) / t36 / t50;
            let t53 = t48 * t52;
            let t56 = t30 * t30;
            let t57 = f64x8::splat(1.0) / t56;
            let t58 = param_a3 * t57;
            let t59 = t47 * v_sigma0;
            let t60 = t49 * t49;
            let t61 = f64x8::splat(1.0) / t60;
            let t62 = t59 * t61;
            let t65 = f64x8::splat(1.0) + t29 * t40 / f64x8::splat(24.0) + t44 * t53 / f64x8::splat(576.0) + t58 * t62 / f64x8::splat(2304.0);
            let t66 = t27 * t65;
            let t67 = param_b1 * t28;
            let t70 = param_b2 * t43;
            let t73 = param_b3 * t57;
            let t76 = f64x8::splat(1.0) + t67 * t40 / f64x8::splat(24.0) + t70 * t53 / f64x8::splat(576.0) + t73 * t62 / f64x8::splat(2304.0);
            let t77 = f64x8::splat(1.0) / t76;
            let t78 = t66 * t77;
            let t81 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t78));
            let t82 = (v_rho1).simd_le(dens_threshold);
            let t83 = -t16;
            let t85 = ((t14).select(t11, (t10).select(t15, t83 * t7)));
            let t86 = f64x8::splat(1.0) + t85;
            let t87 = (t86).simd_le(zeta_threshold);
            let t88 = (simd::cbrt(t86));
            let t90 = ((t87).select(t22, t88 * t86));
            let t91 = t5 * t90;
            let t92 = t33 * v_sigma2;
            let t93 = v_rho1 * v_rho1;
            let t94 = (simd::cbrt(v_rho1));
            let t95 = t94 * t94;
            let t97 = f64x8::splat(1.0) / t95 / t93;
            let t98 = t92 * t97;
            let t101 = v_sigma2 * v_sigma2;
            let t102 = t46 * t101;
            let t103 = t93 * t93;
            let t104 = t103 * v_rho1;
            let t106 = f64x8::splat(1.0) / t94 / t104;
            let t107 = t102 * t106;
            let t110 = t101 * v_sigma2;
            let t111 = t103 * t103;
            let t112 = f64x8::splat(1.0) / t111;
            let t113 = t110 * t112;
            let t116 = f64x8::splat(1.0) + t29 * t98 / f64x8::splat(24.0) + t44 * t107 / f64x8::splat(576.0) + t58 * t113 / f64x8::splat(2304.0);
            let t117 = t27 * t116;
            let t124 = f64x8::splat(1.0) + t67 * t98 / f64x8::splat(24.0) + t70 * t107 / f64x8::splat(576.0) + t73 * t113 / f64x8::splat(2304.0);
            let t125 = f64x8::splat(1.0) / t124;
            let t126 = t117 * t125;
            let t129 = ((t82).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t126));
            let tzk0 = t81 + t129;
            acc_zk = tzk0;
            let t130 = t6 * t6;
            let t131 = f64x8::splat(1.0) / t130;
            let t132 = t16 * t131;
            let t134 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t132)));
            let t137 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t134));
            let t138 = t5 * t137;
            let t141 = t27 * t27;
            let t142 = f64x8::splat(1.0) / t141;
            let t143 = t142 * t65;
            let t144 = t143 * t77;
            let t146 = t26 * t144 / f64x8::splat(8.0);
            let t147 = t35 * v_rho0;
            let t149 = f64x8::splat(1.0) / t37 / t147;
            let t150 = t34 * t149;
            let t153 = t49 * t35;
            let t155 = f64x8::splat(1.0) / t36 / t153;
            let t156 = t48 * t155;
            let t159 = t60 * v_rho0;
            let t160 = f64x8::splat(1.0) / t159;
            let t161 = t59 * t160;
            let t164 = -t29 * t150 / f64x8::splat(9.0) - t44 * t156 / f64x8::splat(108.0) - t58 * t161 / f64x8::splat(288.0);
            let t165 = t27 * t164;
            let t166 = t165 * t77;
            let t169 = t76 * t76;
            let t170 = f64x8::splat(1.0) / t169;
            let t177 = -t67 * t150 / f64x8::splat(9.0) - t70 * t156 / f64x8::splat(108.0) - t73 * t161 / f64x8::splat(288.0);
            let t178 = t170 * t177;
            let t179 = t66 * t178;
            let t183 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t138 * t78 - t146 - f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t166 + f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t179));
            let t184 = t83 * t131;
            let t186 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t184)));
            let t189 = ((t87).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t88 * t186));
            let t190 = t5 * t189;
            let t193 = t142 * t116;
            let t194 = t193 * t125;
            let t196 = t91 * t194 / f64x8::splat(8.0);
            let t198 = ((t82).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t190 * t126 - t196));
            let tvrho0 = t81 + t129 + t6 * (t183 + t198);
            acc_vrho_0 = tvrho0;
            let t202 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t132)));
            let t205 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t202));
            let t206 = t5 * t205;
            let t210 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t206 * t78 - t146));
            let t212 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t184)));
            let t215 = ((t87).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t88 * t212));
            let t216 = t5 * t215;
            let t219 = t93 * v_rho1;
            let t221 = f64x8::splat(1.0) / t95 / t219;
            let t222 = t92 * t221;
            let t225 = t103 * t93;
            let t227 = f64x8::splat(1.0) / t94 / t225;
            let t228 = t102 * t227;
            let t231 = t111 * v_rho1;
            let t232 = f64x8::splat(1.0) / t231;
            let t233 = t110 * t232;
            let t236 = -t29 * t222 / f64x8::splat(9.0) - t44 * t228 / f64x8::splat(108.0) - t58 * t233 / f64x8::splat(288.0);
            let t237 = t27 * t236;
            let t238 = t237 * t125;
            let t241 = t124 * t124;
            let t242 = f64x8::splat(1.0) / t241;
            let t249 = -t67 * t222 / f64x8::splat(9.0) - t70 * t228 / f64x8::splat(108.0) - t73 * t233 / f64x8::splat(288.0);
            let t250 = t242 * t249;
            let t251 = t117 * t250;
            let t255 = ((t82).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t216 * t126 - t196 - f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t238 + f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t251));
            let tvrho1 = t81 + t129 + t6 * (t210 + t255);
            acc_vrho_1 = tvrho1;
            let t258 = t33 * t39;
            let t261 = t46 * v_sigma0;
            let t262 = t261 * t52;
            let t265 = t47 * t61;
            let t268 = t29 * t258 / f64x8::splat(24.0) + t44 * t262 / f64x8::splat(288.0) + t58 * t265 / f64x8::splat(768.0);
            let t269 = t27 * t268;
            let t270 = t269 * t77;
            let t278 = t67 * t258 / f64x8::splat(24.0) + t70 * t262 / f64x8::splat(288.0) + t73 * t265 / f64x8::splat(768.0);
            let t279 = t170 * t278;
            let t280 = t66 * t279;
            let t284 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t270 + f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t280));
            let tvsigma0 = t6 * t284;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t285 = t33 * t97;
            let t288 = t46 * v_sigma2;
            let t289 = t288 * t106;
            let t292 = t101 * t112;
            let t295 = t29 * t285 / f64x8::splat(24.0) + t44 * t289 / f64x8::splat(288.0) + t58 * t292 / f64x8::splat(768.0);
            let t296 = t27 * t295;
            let t297 = t296 * t125;
            let t305 = t67 * t285 / f64x8::splat(24.0) + t70 * t289 / f64x8::splat(288.0) + t73 * t292 / f64x8::splat(768.0);
            let t306 = t242 * t305;
            let t307 = t117 * t306;
            let t311 = ((t82).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t297 + f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t307));
            let tvsigma2 = t6 * t311;
            acc_vsigma_2 = tvsigma2;
            let t314 = t23 * t23;
            let t315 = f64x8::splat(1.0) / t314;
            let t316 = t134 * t134;
            let t319 = t130 * t6;
            let t320 = f64x8::splat(1.0) / t319;
            let t321 = t16 * t320;
            let t324 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t131 + f64x8::splat(2.0) * t321)));
            let t328 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t315 * t316 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t324));
            let t329 = t5 * t328;
            let t332 = t138 * t144;
            let t339 = f64x8::splat(1.0) / t141 / t6;
            let t340 = t339 * t65;
            let t341 = t340 * t77;
            let t343 = t26 * t341 / f64x8::splat(12.0);
            let t344 = t142 * t164;
            let t345 = t344 * t77;
            let t346 = t26 * t345;
            let t348 = t143 * t178;
            let t349 = t26 * t348;
            let t352 = f64x8::splat(1.0) / t37 / t49;
            let t353 = t34 * t352;
            let t358 = f64x8::splat(1.0) / t36 / t49 / t147;
            let t359 = t48 * t358;
            let t363 = f64x8::splat(1.0) / t60 / t35;
            let t364 = t59 * t363;
            let t367 = f64x8::splat(11.0) / f64x8::splat(27.0) * t29 * t353 + f64x8::splat(19.0) / f64x8::splat(324.0) * t44 * t359 + t58 * t364 / f64x8::splat(32.0);
            let t368 = t27 * t367;
            let t369 = t368 * t77;
            let t372 = t165 * t178;
            let t376 = f64x8::splat(1.0) / t169 / t76;
            let t377 = t177 * t177;
            let t378 = t376 * t377;
            let t379 = t66 * t378;
            let t388 = f64x8::splat(11.0) / f64x8::splat(27.0) * t67 * t353 + f64x8::splat(19.0) / f64x8::splat(324.0) * t70 * t359 + t73 * t364 / f64x8::splat(32.0);
            let t389 = t170 * t388;
            let t390 = t66 * t389;
            let t393 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t329 * t78 - t332 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t138 * t166 + f64x8::splat(3.0) / f64x8::splat(4.0) * t138 * t179 + t343 - t346 / f64x8::splat(4.0) + t349 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t369 + f64x8::splat(3.0) / f64x8::splat(4.0) * t26 * t372 - f64x8::splat(3.0) / f64x8::splat(4.0) * t26 * t379 + f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t390;
            let t394 = ((t1).select(f64x8::splat(0.0), t393));
            let t395 = t88 * t88;
            let t396 = f64x8::splat(1.0) / t395;
            let t397 = t186 * t186;
            let t400 = t83 * t320;
            let t403 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t131 + f64x8::splat(2.0) * t400)));
            let t407 = ((t87).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t396 * t397 + f64x8::splat(4.0) / f64x8::splat(3.0) * t88 * t403));
            let t408 = t5 * t407;
            let t411 = t190 * t194;
            let t413 = t339 * t116;
            let t414 = t413 * t125;
            let t416 = t91 * t414 / f64x8::splat(12.0);
            let t418 = ((t82).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t408 * t126 - t411 / f64x8::splat(4.0) + t416));
            let tv2rho20 = f64x8::splat(2.0) * t183 + f64x8::splat(2.0) * t198 + t6 * (t394 + t418);
            acc_v2rho2_0 = tv2rho20;
            let t421 = t315 * t202;
            let t425 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t321)));
            let t429 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t421 * t134 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t425));
            let t430 = t5 * t429;
            let t433 = t206 * t144;
            let t443 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t430 * t78 - t433 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t206 * t166 + f64x8::splat(3.0) / f64x8::splat(8.0) * t206 * t179 - t332 / f64x8::splat(8.0) + t343 - t346 / f64x8::splat(8.0) + t349 / f64x8::splat(8.0)));
            let t444 = t396 * t212;
            let t448 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t400)));
            let t452 = ((t87).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t444 * t186 + f64x8::splat(4.0) / f64x8::splat(3.0) * t88 * t448));
            let t453 = t5 * t452;
            let t456 = t216 * t194;
            let t461 = t142 * t236;
            let t462 = t461 * t125;
            let t463 = t91 * t462;
            let t467 = t193 * t250;
            let t468 = t91 * t467;
            let t471 = ((t82).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t453 * t126 - t456 / f64x8::splat(8.0) - t411 / f64x8::splat(8.0) + t416 - f64x8::splat(3.0) / f64x8::splat(8.0) * t190 * t238 - t463 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(8.0) * t190 * t251 + t468 / f64x8::splat(8.0)));
            let tv2rho21 = t183 + t198 + t210 + t255 + t6 * (t443 + t471);
            acc_v2rho2_1 = tv2rho21;
            let t476 = t202 * t202;
            let t481 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t131 + f64x8::splat(2.0) * t321)));
            let t485 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t315 * t476 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t481));
            let t486 = t5 * t485;
            let t491 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t486 * t78 - t433 / f64x8::splat(4.0) + t343));
            let t492 = t212 * t212;
            let t497 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t131 + f64x8::splat(2.0) * t400)));
            let t501 = ((t87).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t396 * t492 + f64x8::splat(4.0) / f64x8::splat(3.0) * t88 * t497));
            let t502 = t5 * t501;
            let t513 = f64x8::splat(1.0) / t95 / t103;
            let t514 = t92 * t513;
            let t519 = f64x8::splat(1.0) / t94 / t103 / t219;
            let t520 = t102 * t519;
            let t524 = f64x8::splat(1.0) / t111 / t93;
            let t525 = t110 * t524;
            let t528 = f64x8::splat(11.0) / f64x8::splat(27.0) * t29 * t514 + f64x8::splat(19.0) / f64x8::splat(324.0) * t44 * t520 + t58 * t525 / f64x8::splat(32.0);
            let t529 = t27 * t528;
            let t530 = t529 * t125;
            let t533 = t237 * t250;
            let t537 = f64x8::splat(1.0) / t241 / t124;
            let t538 = t249 * t249;
            let t539 = t537 * t538;
            let t540 = t117 * t539;
            let t549 = f64x8::splat(11.0) / f64x8::splat(27.0) * t67 * t514 + f64x8::splat(19.0) / f64x8::splat(324.0) * t70 * t520 + t73 * t525 / f64x8::splat(32.0);
            let t550 = t242 * t549;
            let t551 = t117 * t550;
            let t554 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t502 * t126 - t456 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t216 * t238 + f64x8::splat(3.0) / f64x8::splat(4.0) * t216 * t251 + t416 - t463 / f64x8::splat(4.0) + t468 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t530 + f64x8::splat(3.0) / f64x8::splat(4.0) * t91 * t533 - f64x8::splat(3.0) / f64x8::splat(4.0) * t91 * t540 + f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t551;
            let t555 = ((t82).select(f64x8::splat(0.0), t554));
            let tv2rho22 = f64x8::splat(2.0) * t210 + f64x8::splat(2.0) * t255 + t6 * (t491 + t555);
            acc_v2rho2_2 = tv2rho22;
            let t560 = t142 * t268;
            let t561 = t560 * t77;
            let t563 = t26 * t561 / f64x8::splat(8.0);
            let t564 = t33 * t149;
            let t567 = t261 * t155;
            let t570 = t47 * t160;
            let t573 = -t29 * t564 / f64x8::splat(9.0) - t44 * t567 / f64x8::splat(54.0) - t58 * t570 / f64x8::splat(96.0);
            let t574 = t27 * t573;
            let t575 = t574 * t77;
            let t578 = t269 * t178;
            let t583 = t143 * t279;
            let t585 = t26 * t583 / f64x8::splat(8.0);
            let t586 = t165 * t279;
            let t589 = t25 * t27;
            let t590 = t5 * t589;
            let t591 = t65 * t376;
            let t592 = t278 * t177;
            let t593 = t591 * t592;
            let t602 = -t67 * t564 / f64x8::splat(9.0) - t70 * t567 / f64x8::splat(54.0) - t73 * t570 / f64x8::splat(96.0);
            let t603 = t170 * t602;
            let t604 = t66 * t603;
            let t608 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t138 * t270 - t563 - f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t575 + f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t578 + f64x8::splat(3.0) / f64x8::splat(8.0) * t138 * t280 + t585 + f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t586 - f64x8::splat(3.0) / f64x8::splat(4.0) * t590 * t593 + f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t604));
            let tv2rhosigma0 = t6 * t608 + t284;
            acc_v2rhosigma_0 = tv2rhosigma0;
            let tv2rhosigma1 = f64x8::splat(0.0);
            acc_v2rhosigma_1 = tv2rhosigma1;
            let t612 = t142 * t295;
            let t613 = t612 * t125;
            let t615 = t91 * t613 / f64x8::splat(8.0);
            let t618 = t193 * t306;
            let t620 = t91 * t618 / f64x8::splat(8.0);
            let t622 = ((t82).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t190 * t297 - t615 + f64x8::splat(3.0) / f64x8::splat(8.0) * t190 * t307 + t620));
            let tv2rhosigma2 = t6 * t622 + t311;
            acc_v2rhosigma_2 = tv2rhosigma2;
            let t629 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t206 * t270 - t563 + f64x8::splat(3.0) / f64x8::splat(8.0) * t206 * t280 + t585));
            let tv2rhosigma3 = t6 * t629 + t284;
            acc_v2rhosigma_3 = tv2rhosigma3;
            let tv2rhosigma4 = f64x8::splat(0.0);
            acc_v2rhosigma_4 = tv2rhosigma4;
            let t633 = t33 * t221;
            let t636 = t288 * t227;
            let t639 = t101 * t232;
            let t642 = -t29 * t633 / f64x8::splat(9.0) - t44 * t636 / f64x8::splat(54.0) - t58 * t639 / f64x8::splat(96.0);
            let t643 = t27 * t642;
            let t644 = t643 * t125;
            let t647 = t296 * t250;
            let t652 = t237 * t306;
            let t655 = t90 * t27;
            let t656 = t5 * t655;
            let t657 = t116 * t537;
            let t658 = t305 * t249;
            let t659 = t657 * t658;
            let t668 = -t67 * t633 / f64x8::splat(9.0) - t70 * t636 / f64x8::splat(54.0) - t73 * t639 / f64x8::splat(96.0);
            let t669 = t242 * t668;
            let t670 = t117 * t669;
            let t674 = ((t82).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t216 * t297 - t615 - f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t644 + f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t647 + f64x8::splat(3.0) / f64x8::splat(8.0) * t216 * t307 + t620 + f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t652 - f64x8::splat(3.0) / f64x8::splat(4.0) * t656 * t659 + f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t670));
            let tv2rhosigma5 = t6 * t674 + t311;
            acc_v2rhosigma_5 = tv2rhosigma5;
            let t676 = t46 * t52;
            let t679 = v_sigma0 * t61;
            let t682 = t44 * t676 / f64x8::splat(288.0) + t58 * t679 / f64x8::splat(384.0);
            let t683 = t27 * t682;
            let t684 = t683 * t77;
            let t687 = t269 * t279;
            let t690 = t278 * t278;
            let t691 = t376 * t690;
            let t692 = t66 * t691;
            let t699 = t70 * t676 / f64x8::splat(288.0) + t73 * t679 / f64x8::splat(384.0);
            let t700 = t170 * t699;
            let t701 = t66 * t700;
            let t705 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t684 + f64x8::splat(3.0) / f64x8::splat(4.0) * t26 * t687 - f64x8::splat(3.0) / f64x8::splat(4.0) * t26 * t692 + f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t701));
            let tv2sigma20 = t6 * t705;
            acc_v2sigma2_0 = tv2sigma20;
            let tv2sigma21 = f64x8::splat(0.0);
            acc_v2sigma2_1 = tv2sigma21;
            let tv2sigma22 = f64x8::splat(0.0);
            acc_v2sigma2_2 = tv2sigma22;
            let tv2sigma23 = f64x8::splat(0.0);
            acc_v2sigma2_3 = tv2sigma23;
            let tv2sigma24 = f64x8::splat(0.0);
            acc_v2sigma2_4 = tv2sigma24;
            let t706 = t46 * t106;
            let t709 = v_sigma2 * t112;
            let t712 = t44 * t706 / f64x8::splat(288.0) + t58 * t709 / f64x8::splat(384.0);
            let t713 = t27 * t712;
            let t714 = t713 * t125;
            let t717 = t296 * t306;
            let t720 = t305 * t305;
            let t721 = t537 * t720;
            let t722 = t117 * t721;
            let t729 = t70 * t706 / f64x8::splat(288.0) + t73 * t709 / f64x8::splat(384.0);
            let t730 = t242 * t729;
            let t731 = t117 * t730;
            let t735 = ((t82).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t714 + f64x8::splat(3.0) / f64x8::splat(4.0) * t91 * t717 - f64x8::splat(3.0) / f64x8::splat(4.0) * t91 * t722 + f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t731));
            let tv2sigma25 = t6 * t735;
            acc_v2sigma2_5 = tv2sigma25;
            let t739 = f64x8::splat(1.0) / t141 / t130;
            let t740 = t739 * t65;
            let t741 = t740 * t77;
            let t743 = f64x8::splat(5.0) / f64x8::splat(36.0) * t26 * t741;
            let t744 = t329 * t144;
            let t746 = t138 * t341;
            let t749 = f64x8::splat(1.0) / t314 / t19;
            let t750 = t316 * t134;
            let t753 = t315 * t134;
            let t756 = t130 * t130;
            let t757 = f64x8::splat(1.0) / t756;
            let t758 = t16 * t757;
            let t761 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(6.0) * t320 - f64x8::splat(6.0) * t758)));
            let t765 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t749 * t750 + f64x8::splat(4.0) / f64x8::splat(3.0) * t753 * t324 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t761));
            let t766 = t5 * t765;
            let t771 = t138 * t345;
            let t775 = t339 * t164;
            let t776 = t775 * t77;
            let t777 = t26 * t776;
            let t779 = t142 * t367;
            let t780 = t779 * t77;
            let t781 = t26 * t780;
            let t784 = f64x8::splat(1.0) / t37 / t50;
            let t785 = t34 * t784;
            let t789 = f64x8::splat(1.0) / t36 / t60;
            let t790 = t48 * t789;
            let t794 = f64x8::splat(1.0) / t60 / t147;
            let t795 = t59 * t794;
            let t798 = -f64x8::splat(154.0) / f64x8::splat(81.0) * t29 * t785 - f64x8::splat(209.0) / f64x8::splat(486.0) * t44 * t790 - f64x8::splat(5.0) / f64x8::splat(16.0) * t58 * t795;
            let t799 = t27 * t798;
            let t800 = t799 * t77;
            let t803 = t177 * t388;
            let t804 = t591 * t803;
            let t807 = t169 * t169;
            let t808 = f64x8::splat(1.0) / t807;
            let t809 = t377 * t177;
            let t810 = t808 * t809;
            let t811 = t66 * t810;
            let t814 = -t743 - f64x8::splat(3.0) / f64x8::splat(8.0) * t744 + t746 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t766 * t78 - f64x8::splat(9.0) / f64x8::splat(8.0) * t329 * t166 - f64x8::splat(3.0) / f64x8::splat(4.0) * t771 - f64x8::splat(9.0) / f64x8::splat(8.0) * t138 * t369 + t777 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t781 - f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t800 - f64x8::splat(9.0) / f64x8::splat(4.0) * t590 * t804 + f64x8::splat(9.0) / f64x8::splat(4.0) * t26 * t811;
            let t817 = t143 * t378;
            let t818 = t26 * t817;
            let t820 = t165 * t378;
            let t823 = t143 * t389;
            let t824 = t26 * t823;
            let t826 = t368 * t178;
            let t829 = t165 * t389;
            let t838 = -f64x8::splat(154.0) / f64x8::splat(81.0) * t67 * t785 - f64x8::splat(209.0) / f64x8::splat(486.0) * t70 * t790 - f64x8::splat(5.0) / f64x8::splat(16.0) * t73 * t795;
            let t839 = t170 * t838;
            let t840 = t66 * t839;
            let t845 = t138 * t348;
            let t851 = t340 * t178;
            let t852 = t26 * t851;
            let t854 = t344 * t178;
            let t855 = t26 * t854;
            let t857 = -f64x8::splat(9.0) / f64x8::splat(4.0) * t138 * t379 - f64x8::splat(3.0) / f64x8::splat(4.0) * t818 - f64x8::splat(9.0) / f64x8::splat(4.0) * t26 * t820 + f64x8::splat(3.0) / f64x8::splat(8.0) * t824 + f64x8::splat(9.0) / f64x8::splat(8.0) * t26 * t826 + f64x8::splat(9.0) / f64x8::splat(8.0) * t26 * t829 + f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t840 + f64x8::splat(9.0) / f64x8::splat(8.0) * t329 * t179 + f64x8::splat(3.0) / f64x8::splat(4.0) * t845 + f64x8::splat(9.0) / f64x8::splat(4.0) * t138 * t372 + f64x8::splat(9.0) / f64x8::splat(8.0) * t138 * t390 - t852 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t855;
            let t859 = ((t1).select(f64x8::splat(0.0), t814 + t857));
            let t861 = f64x8::splat(1.0) / t395 / t86;
            let t862 = t397 * t186;
            let t865 = t396 * t186;
            let t868 = t83 * t757;
            let t871 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t320 - f64x8::splat(6.0) * t868)));
            let t875 = ((t87).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t861 * t862 + f64x8::splat(4.0) / f64x8::splat(3.0) * t865 * t403 + f64x8::splat(4.0) / f64x8::splat(3.0) * t88 * t871));
            let t876 = t5 * t875;
            let t879 = t408 * t194;
            let t881 = t190 * t414;
            let t883 = t739 * t116;
            let t884 = t883 * t125;
            let t886 = f64x8::splat(5.0) / f64x8::splat(36.0) * t91 * t884;
            let t888 = ((t82).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t876 * t126 - f64x8::splat(3.0) / f64x8::splat(8.0) * t879 + t881 / f64x8::splat(4.0) - t886));
            let tv3rho30 = f64x8::splat(3.0) * t394 + f64x8::splat(3.0) * t418 + t6 * (t859 + t888);
            acc_v3rho3_0 = tv3rho30;
            let t891 = f64x8::splat(2.0) * t443;
            let t892 = f64x8::splat(2.0) * t471;
            let t893 = t749 * t202;
            let t896 = t315 * t425;
            let t901 = f64x8::splat(2.0) * t320;
            let t902 = f64x8::splat(6.0) * t758;
            let t904 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t901 - t902)));
            let t908 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t893 * t316 + f64x8::splat(8.0) / f64x8::splat(9.0) * t896 * t134 + f64x8::splat(4.0) / f64x8::splat(9.0) * t421 * t324 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t904));
            let t909 = t5 * t908;
            let t915 = t206 * t345 / f64x8::splat(4.0);
            let t925 = t430 * t144 / f64x8::splat(4.0);
            let t928 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t909 * t78 - f64x8::splat(3.0) / f64x8::splat(4.0) * t430 * t166 - t915 - f64x8::splat(3.0) / f64x8::splat(8.0) * t206 * t369 - t771 / f64x8::splat(4.0) + t777 / f64x8::splat(6.0) - t781 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t206 * t379 - t818 / f64x8::splat(4.0) - t925 + f64x8::splat(3.0) / f64x8::splat(4.0) * t430 * t179;
            let t929 = t206 * t341;
            let t932 = t206 * t348 / f64x8::splat(4.0);
            let t943 = t929 / f64x8::splat(12.0) + t932 + f64x8::splat(3.0) / f64x8::splat(4.0) * t206 * t372 + f64x8::splat(3.0) / f64x8::splat(8.0) * t206 * t390 + t824 / f64x8::splat(8.0) - t744 / f64x8::splat(8.0) + t746 / f64x8::splat(6.0) + t845 / f64x8::splat(4.0) - t743 - t852 / f64x8::splat(6.0) + t855 / f64x8::splat(4.0);
            let t945 = ((t1).select(f64x8::splat(0.0), t928 + t943));
            let t946 = t861 * t212;
            let t949 = t396 * t448;
            let t954 = f64x8::splat(6.0) * t868;
            let t956 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t901 - t954)));
            let t960 = ((t87).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t946 * t397 + f64x8::splat(8.0) / f64x8::splat(9.0) * t949 * t186 + f64x8::splat(4.0) / f64x8::splat(9.0) * t444 * t403 + f64x8::splat(4.0) / f64x8::splat(3.0) * t88 * t956));
            let t961 = t5 * t960;
            let t965 = t453 * t194 / f64x8::splat(4.0);
            let t966 = t216 * t414;
            let t973 = t190 * t462 / f64x8::splat(4.0);
            let t974 = t339 * t236;
            let t975 = t974 * t125;
            let t976 = t91 * t975;
            let t981 = t190 * t467 / f64x8::splat(4.0);
            let t982 = t413 * t250;
            let t983 = t91 * t982;
            let t985 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t961 * t126 - t965 + t966 / f64x8::splat(12.0) - t879 / f64x8::splat(8.0) + t881 / f64x8::splat(6.0) - t886 - f64x8::splat(3.0) / f64x8::splat(8.0) * t408 * t238 - t973 + t976 / f64x8::splat(12.0) + f64x8::splat(3.0) / f64x8::splat(8.0) * t408 * t251 + t981 - t983 / f64x8::splat(12.0);
            let t986 = ((t82).select(f64x8::splat(0.0), t985));
            let tv3rho31 = t394 + t418 + t891 + t892 + t6 * (t945 + t986);
            acc_v3rho3_1 = tv3rho31;
            let t989 = t749 * t476;
            let t994 = t315 * t481;
            let t998 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t901 - t902)));
            let t1002 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t989 * t134 + f64x8::splat(8.0) / f64x8::splat(9.0) * t421 * t425 + f64x8::splat(4.0) / f64x8::splat(9.0) * t994 * t134 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t998));
            let t1003 = t5 * t1002;
            let t1006 = t486 * t144;
            let t1016 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t1003 * t78 - t1006 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t486 * t166 + f64x8::splat(3.0) / f64x8::splat(8.0) * t486 * t179 - t925 + t929 / f64x8::splat(6.0) - t915 + t932 + t746 / f64x8::splat(12.0) - t743 + t777 / f64x8::splat(12.0) - t852 / f64x8::splat(12.0);
            let t1017 = ((t1).select(f64x8::splat(0.0), t1016));
            let t1018 = t861 * t492;
            let t1023 = t396 * t497;
            let t1027 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t901 - t954)));
            let t1031 = ((t87).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t1018 * t186 + f64x8::splat(8.0) / f64x8::splat(9.0) * t444 * t448 + f64x8::splat(4.0) / f64x8::splat(9.0) * t1023 * t186 + f64x8::splat(4.0) / f64x8::splat(3.0) * t88 * t1027));
            let t1032 = t5 * t1031;
            let t1035 = t502 * t194;
            let t1040 = t216 * t462;
            let t1044 = t216 * t467;
            let t1047 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t1032 * t126 - t1035 / f64x8::splat(8.0) - t965 + t966 / f64x8::splat(6.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t453 * t238 - t1040 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t453 * t251 + t1044 / f64x8::splat(4.0) + t881 / f64x8::splat(12.0) - t886 - t973;
            let t1052 = t142 * t528;
            let t1053 = t1052 * t125;
            let t1054 = t91 * t1053;
            let t1058 = t461 * t250;
            let t1059 = t91 * t1058;
            let t1063 = t193 * t539;
            let t1064 = t91 * t1063;
            let t1068 = t193 * t550;
            let t1069 = t91 * t1068;
            let t1071 = t976 / f64x8::splat(6.0) + t981 - t983 / f64x8::splat(6.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t190 * t530 - t1054 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t190 * t533 + t1059 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t190 * t540 - t1064 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(8.0) * t190 * t551 + t1069 / f64x8::splat(8.0);
            let t1073 = ((t82).select(f64x8::splat(0.0), t1047 + t1071));
            let tv3rho32 = t891 + t892 + t491 + t555 + t6 * (t1017 + t1073);
            acc_v3rho3_2 = tv3rho32;
            let t1078 = t476 * t202;
            let t1085 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t320 - f64x8::splat(6.0) * t758)));
            let t1089 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t749 * t1078 + f64x8::splat(4.0) / f64x8::splat(3.0) * t421 * t481 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t1085));
            let t1090 = t5 * t1089;
            let t1096 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t1090 * t78 - f64x8::splat(3.0) / f64x8::splat(8.0) * t1006 + t929 / f64x8::splat(4.0) - t743));
            let t1100 = t237 * t539;
            let t1103 = t241 * t241;
            let t1104 = f64x8::splat(1.0) / t1103;
            let t1105 = t538 * t249;
            let t1106 = t1104 * t1105;
            let t1107 = t117 * t1106;
            let t1110 = t529 * t250;
            let t1113 = t237 * t550;
            let t1117 = f64x8::splat(1.0) / t95 / t104;
            let t1118 = t92 * t1117;
            let t1122 = f64x8::splat(1.0) / t94 / t111;
            let t1123 = t102 * t1122;
            let t1127 = f64x8::splat(1.0) / t111 / t219;
            let t1128 = t110 * t1127;
            let t1131 = -f64x8::splat(154.0) / f64x8::splat(81.0) * t67 * t1118 - f64x8::splat(209.0) / f64x8::splat(486.0) * t70 * t1123 - f64x8::splat(5.0) / f64x8::splat(16.0) * t73 * t1128;
            let t1132 = t242 * t1131;
            let t1133 = t117 * t1132;
            let t1144 = f64x8::splat(3.0) / f64x8::splat(8.0) * t1069 - f64x8::splat(9.0) / f64x8::splat(4.0) * t216 * t540 - f64x8::splat(9.0) / f64x8::splat(4.0) * t91 * t1100 + f64x8::splat(9.0) / f64x8::splat(4.0) * t91 * t1107 + f64x8::splat(9.0) / f64x8::splat(8.0) * t91 * t1110 + f64x8::splat(9.0) / f64x8::splat(8.0) * t91 * t1113 + f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t1133 + f64x8::splat(9.0) / f64x8::splat(8.0) * t502 * t251 + f64x8::splat(9.0) / f64x8::splat(4.0) * t216 * t533 + f64x8::splat(9.0) / f64x8::splat(8.0) * t216 * t551 - t983 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t1044;
            let t1153 = -f64x8::splat(154.0) / f64x8::splat(81.0) * t29 * t1118 - f64x8::splat(209.0) / f64x8::splat(486.0) * t44 * t1123 - f64x8::splat(5.0) / f64x8::splat(16.0) * t58 * t1128;
            let t1154 = t27 * t1153;
            let t1155 = t1154 * t125;
            let t1158 = t492 * t212;
            let t1165 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(6.0) * t320 - f64x8::splat(6.0) * t868)));
            let t1169 = ((t87).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t861 * t1158 + f64x8::splat(4.0) / f64x8::splat(3.0) * t444 * t497 + f64x8::splat(4.0) / f64x8::splat(3.0) * t88 * t1165));
            let t1170 = t5 * t1169;
            let t1182 = t249 * t549;
            let t1183 = t657 * t1182;
            let t1186 = f64x8::splat(3.0) / f64x8::splat(4.0) * t1059 - f64x8::splat(3.0) / f64x8::splat(4.0) * t1064 - f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t1155 - f64x8::splat(3.0) / f64x8::splat(8.0) * t1170 * t126 - f64x8::splat(9.0) / f64x8::splat(8.0) * t502 * t238 - f64x8::splat(9.0) / f64x8::splat(8.0) * t216 * t530 - f64x8::splat(3.0) / f64x8::splat(8.0) * t1054 - f64x8::splat(3.0) / f64x8::splat(4.0) * t1040 - f64x8::splat(3.0) / f64x8::splat(8.0) * t1035 + t976 / f64x8::splat(4.0) + t966 / f64x8::splat(4.0) - t886 - f64x8::splat(9.0) / f64x8::splat(4.0) * t656 * t1183;
            let t1188 = ((t82).select(f64x8::splat(0.0), t1144 + t1186));
            let tv3rho33 = f64x8::splat(3.0) * t491 + f64x8::splat(3.0) * t555 + t6 * (t1096 + t1188);
            acc_v3rho3_3 = tv3rho33;
            let t1192 = t269 * t378;
            let t1195 = t138 * t583;
            let t1197 = t340 * t279;
            let t1199 = t26 * t1197 / f64x8::splat(12.0);
            let t1200 = t560 * t178;
            let t1201 = t26 * t1200;
            let t1203 = t574 * t178;
            let t1206 = t269 * t389;
            let t1215 = t344 * t279;
            let t1216 = t26 * t1215;
            let t1218 = t143 * t603;
            let t1219 = t26 * t1218;
            let t1221 = t368 * t279;
            let t1224 = t165 * t603;
            let t1227 = -f64x8::splat(3.0) / f64x8::splat(4.0) * t26 * t1192 + t1195 / f64x8::splat(4.0) - t1199 + t1201 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t26 * t1203 + f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t1206 + f64x8::splat(3.0) / f64x8::splat(8.0) * t329 * t280 + f64x8::splat(3.0) / f64x8::splat(4.0) * t138 * t586 + f64x8::splat(3.0) / f64x8::splat(4.0) * t138 * t604 + t1216 / f64x8::splat(4.0) + t1219 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t1221 + f64x8::splat(3.0) / f64x8::splat(4.0) * t26 * t1224;
            let t1228 = t33 * t352;
            let t1231 = t261 * t358;
            let t1234 = t47 * t363;
            let t1237 = f64x8::splat(11.0) / f64x8::splat(27.0) * t67 * t1228 + f64x8::splat(19.0) / f64x8::splat(162.0) * t70 * t1231 + f64x8::splat(3.0) / f64x8::splat(32.0) * t73 * t1234;
            let t1238 = t170 * t1237;
            let t1239 = t66 * t1238;
            let t1244 = t138 * t561;
            let t1246 = t339 * t268;
            let t1247 = t1246 * t77;
            let t1249 = t26 * t1247 / f64x8::splat(12.0);
            let t1252 = t142 * t573;
            let t1253 = t1252 * t77;
            let t1254 = t26 * t1253;
            let t1262 = f64x8::splat(11.0) / f64x8::splat(27.0) * t29 * t1228 + f64x8::splat(19.0) / f64x8::splat(162.0) * t44 * t1231 + f64x8::splat(3.0) / f64x8::splat(32.0) * t58 * t1234;
            let t1263 = t27 * t1262;
            let t1264 = t1263 * t77;
            let t1269 = t65 * t808;
            let t1270 = t278 * t377;
            let t1271 = t1269 * t1270;
            let t1274 = t137 * t27;
            let t1275 = t5 * t1274;
            let t1278 = t25 * t142;
            let t1279 = t5 * t1278;
            let t1280 = t1279 * t593;
            let t1282 = t164 * t376;
            let t1283 = t1282 * t592;
            let t1286 = t602 * t177;
            let t1287 = t591 * t1286;
            let t1290 = t278 * t388;
            let t1291 = t591 * t1290;
            let t1294 = f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t1239 + f64x8::splat(3.0) / f64x8::splat(4.0) * t138 * t578 - t1244 / f64x8::splat(4.0) + t1249 - f64x8::splat(3.0) / f64x8::splat(4.0) * t138 * t575 - t1254 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t1264 - f64x8::splat(3.0) / f64x8::splat(8.0) * t329 * t270 + f64x8::splat(9.0) / f64x8::splat(4.0) * t590 * t1271 - f64x8::splat(3.0) / f64x8::splat(2.0) * t1275 * t593 - t1280 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(2.0) * t590 * t1283 - f64x8::splat(3.0) / f64x8::splat(2.0) * t590 * t1287 - f64x8::splat(3.0) / f64x8::splat(4.0) * t590 * t1291;
            let t1296 = ((t1).select(f64x8::splat(0.0), t1227 + t1294));
            let tv3rho2sigma0 = t6 * t1296 + f64x8::splat(2.0) * t608;
            acc_v3rho2sigma_0 = tv3rho2sigma0;
            let tv3rho2sigma1 = f64x8::splat(0.0);
            acc_v3rho2sigma_1 = tv3rho2sigma1;
            let t1301 = t190 * t613;
            let t1303 = t339 * t295;
            let t1304 = t1303 * t125;
            let t1306 = t91 * t1304 / f64x8::splat(12.0);
            let t1309 = t190 * t618;
            let t1311 = t413 * t306;
            let t1313 = t91 * t1311 / f64x8::splat(12.0);
            let t1315 = ((t82).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t408 * t297 - t1301 / f64x8::splat(4.0) + t1306 + f64x8::splat(3.0) / f64x8::splat(8.0) * t408 * t307 + t1309 / f64x8::splat(4.0) - t1313));
            let tv3rho2sigma2 = t6 * t1315 + f64x8::splat(2.0) * t622;
            acc_v3rho2sigma_2 = tv3rho2sigma2;
            let t1319 = t206 * t561;
            let t1330 = t206 * t583;
            let t1334 = t205 * t27;
            let t1335 = t5 * t1334;
            let t1344 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t430 * t270 - t1319 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t206 * t575 + f64x8::splat(3.0) / f64x8::splat(8.0) * t206 * t578 - t1244 / f64x8::splat(8.0) + t1249 - t1254 / f64x8::splat(8.0) + t1201 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(8.0) * t430 * t280 + t1330 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(8.0) * t206 * t586 - f64x8::splat(3.0) / f64x8::splat(4.0) * t1335 * t593 + f64x8::splat(3.0) / f64x8::splat(8.0) * t206 * t604 + t1195 / f64x8::splat(8.0) - t1199 + t1216 / f64x8::splat(8.0) - t1280 / f64x8::splat(4.0) + t1219 / f64x8::splat(8.0);
            let t1345 = ((t1).select(f64x8::splat(0.0), t1344));
            let tv3rho2sigma3 = t6 * t1345 + t608 + t629;
            acc_v3rho2sigma_3 = tv3rho2sigma3;
            let tv3rho2sigma4 = f64x8::splat(0.0);
            acc_v3rho2sigma_4 = tv3rho2sigma4;
            let t1349 = t216 * t613;
            let t1354 = t142 * t642;
            let t1355 = t1354 * t125;
            let t1356 = t91 * t1355;
            let t1360 = t612 * t250;
            let t1361 = t91 * t1360;
            let t1365 = t216 * t618;
            let t1370 = t461 * t306;
            let t1371 = t91 * t1370;
            let t1373 = t189 * t27;
            let t1374 = t5 * t1373;
            let t1377 = t90 * t142;
            let t1378 = t5 * t1377;
            let t1379 = t1378 * t659;
            let t1383 = t193 * t669;
            let t1384 = t91 * t1383;
            let t1386 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t453 * t297 - t1349 / f64x8::splat(8.0) - t1301 / f64x8::splat(8.0) + t1306 - f64x8::splat(3.0) / f64x8::splat(8.0) * t190 * t644 - t1356 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(8.0) * t190 * t647 + t1361 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(8.0) * t453 * t307 + t1365 / f64x8::splat(8.0) + t1309 / f64x8::splat(8.0) - t1313 + f64x8::splat(3.0) / f64x8::splat(8.0) * t190 * t652 + t1371 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t1374 * t659 - t1379 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(8.0) * t190 * t670 + t1384 / f64x8::splat(8.0);
            let t1387 = ((t82).select(f64x8::splat(0.0), t1386));
            let tv3rho2sigma5 = t6 * t1387 + t622 + t674;
            acc_v3rho2sigma_5 = tv3rho2sigma5;
            let t1397 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t486 * t270 - t1319 / f64x8::splat(4.0) + t1249 + f64x8::splat(3.0) / f64x8::splat(8.0) * t486 * t280 + t1330 / f64x8::splat(4.0) - t1199));
            let tv3rho2sigma6 = t6 * t1397 + f64x8::splat(2.0) * t629;
            acc_v3rho2sigma_6 = tv3rho2sigma6;
            let tv3rho2sigma7 = f64x8::splat(0.0);
            acc_v3rho2sigma_7 = tv3rho2sigma7;
            let t1404 = t33 * t513;
            let t1407 = t288 * t519;
            let t1410 = t101 * t524;
            let t1413 = f64x8::splat(11.0) / f64x8::splat(27.0) * t29 * t1404 + f64x8::splat(19.0) / f64x8::splat(162.0) * t44 * t1407 + f64x8::splat(3.0) / f64x8::splat(32.0) * t58 * t1410;
            let t1414 = t27 * t1413;
            let t1415 = t1414 * t125;
            let t1420 = t529 * t306;
            let t1423 = t237 * t669;
            let t1426 = t296 * t539;
            let t1439 = f64x8::splat(11.0) / f64x8::splat(27.0) * t67 * t1404 + f64x8::splat(19.0) / f64x8::splat(162.0) * t70 * t1407 + f64x8::splat(3.0) / f64x8::splat(32.0) * t73 * t1410;
            let t1440 = t242 * t1439;
            let t1441 = t117 * t1440;
            let t1444 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t502 * t297 - f64x8::splat(3.0) / f64x8::splat(4.0) * t216 * t644 - f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t1415 - t1349 / f64x8::splat(4.0) - t1356 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t1420 + f64x8::splat(3.0) / f64x8::splat(4.0) * t91 * t1423 - f64x8::splat(3.0) / f64x8::splat(4.0) * t91 * t1426 + t1361 / f64x8::splat(4.0) + t1365 / f64x8::splat(4.0) + t1371 / f64x8::splat(4.0) + t1384 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t1441;
            let t1447 = t643 * t250;
            let t1450 = t296 * t550;
            let t1459 = t215 * t27;
            let t1460 = t5 * t1459;
            let t1463 = t236 * t537;
            let t1464 = t1463 * t658;
            let t1467 = t668 * t249;
            let t1468 = t657 * t1467;
            let t1471 = t305 * t549;
            let t1472 = t657 * t1471;
            let t1475 = t116 * t1104;
            let t1476 = t305 * t538;
            let t1477 = t1475 * t1476;
            let t1481 = f64x8::splat(3.0) / f64x8::splat(4.0) * t216 * t647 + f64x8::splat(3.0) / f64x8::splat(4.0) * t91 * t1447 + f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t1450 + f64x8::splat(3.0) / f64x8::splat(8.0) * t502 * t307 + f64x8::splat(3.0) / f64x8::splat(4.0) * t216 * t652 + f64x8::splat(3.0) / f64x8::splat(4.0) * t216 * t670 - t1313 + t1306 - f64x8::splat(3.0) / f64x8::splat(2.0) * t1460 * t659 - f64x8::splat(3.0) / f64x8::splat(2.0) * t656 * t1464 - f64x8::splat(3.0) / f64x8::splat(2.0) * t656 * t1468 - f64x8::splat(3.0) / f64x8::splat(4.0) * t656 * t1472 + f64x8::splat(9.0) / f64x8::splat(4.0) * t656 * t1477 - t1379 / f64x8::splat(2.0);
            let t1483 = ((t82).select(f64x8::splat(0.0), t1444 + t1481));
            let tv3rho2sigma8 = t6 * t1483 + f64x8::splat(2.0) * t674;
            acc_v3rho2sigma_8 = tv3rho2sigma8;
            let t1487 = t142 * t682;
            let t1488 = t1487 * t77;
            let t1490 = t26 * t1488 / f64x8::splat(8.0);
            let t1491 = t46 * t155;
            let t1494 = v_sigma0 * t160;
            let t1497 = -t44 * t1491 / f64x8::splat(54.0) - t58 * t1494 / f64x8::splat(48.0);
            let t1498 = t27 * t1497;
            let t1499 = t1498 * t77;
            let t1502 = t683 * t178;
            let t1507 = t560 * t279;
            let t1509 = t26 * t1507 / f64x8::splat(4.0);
            let t1510 = t574 * t279;
            let t1513 = t268 * t376;
            let t1514 = t1513 * t592;
            let t1517 = t269 * t603;
            let t1522 = t143 * t691;
            let t1524 = t26 * t1522 / f64x8::splat(4.0);
            let t1525 = t165 * t691;
            let t1528 = t690 * t177;
            let t1529 = t1269 * t1528;
            let t1532 = t278 * t602;
            let t1533 = t591 * t1532;
            let t1538 = t143 * t700;
            let t1540 = t26 * t1538 / f64x8::splat(8.0);
            let t1541 = t165 * t700;
            let t1544 = t699 * t177;
            let t1545 = t591 * t1544;
            let t1552 = -t70 * t1491 / f64x8::splat(54.0) - t73 * t1494 / f64x8::splat(48.0);
            let t1553 = t170 * t1552;
            let t1554 = t66 * t1553;
            let t1557 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t138 * t684 - t1490 - f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t1499 + f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t1502 + f64x8::splat(3.0) / f64x8::splat(4.0) * t138 * t687 + t1509 + f64x8::splat(3.0) / f64x8::splat(4.0) * t26 * t1510 - f64x8::splat(3.0) / f64x8::splat(2.0) * t590 * t1514 + f64x8::splat(3.0) / f64x8::splat(4.0) * t26 * t1517 - f64x8::splat(3.0) / f64x8::splat(4.0) * t138 * t692 - t1524 - f64x8::splat(3.0) / f64x8::splat(4.0) * t26 * t1525 + f64x8::splat(9.0) / f64x8::splat(4.0) * t590 * t1529 - f64x8::splat(3.0) / f64x8::splat(2.0) * t590 * t1533 + f64x8::splat(3.0) / f64x8::splat(8.0) * t138 * t701 + t1540 + f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t1541 - f64x8::splat(3.0) / f64x8::splat(4.0) * t590 * t1545 + f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t1554;
            let t1558 = ((t1).select(f64x8::splat(0.0), t1557));
            let tv3rhosigma20 = t6 * t1558 + t705;
            acc_v3rhosigma2_0 = tv3rhosigma20;
            let tv3rhosigma21 = f64x8::splat(0.0);
            acc_v3rhosigma2_1 = tv3rhosigma21;
            let tv3rhosigma22 = f64x8::splat(0.0);
            acc_v3rhosigma2_2 = tv3rhosigma22;
            let tv3rhosigma23 = f64x8::splat(0.0);
            acc_v3rhosigma2_3 = tv3rhosigma23;
            let tv3rhosigma24 = f64x8::splat(0.0);
            acc_v3rhosigma2_4 = tv3rhosigma24;
            let t1562 = t142 * t712;
            let t1563 = t1562 * t125;
            let t1565 = t91 * t1563 / f64x8::splat(8.0);
            let t1568 = t612 * t306;
            let t1570 = t91 * t1568 / f64x8::splat(4.0);
            let t1573 = t193 * t721;
            let t1575 = t91 * t1573 / f64x8::splat(4.0);
            let t1578 = t193 * t730;
            let t1580 = t91 * t1578 / f64x8::splat(8.0);
            let t1582 = ((t82).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t190 * t714 - t1565 + f64x8::splat(3.0) / f64x8::splat(4.0) * t190 * t717 + t1570 - f64x8::splat(3.0) / f64x8::splat(4.0) * t190 * t722 - t1575 + f64x8::splat(3.0) / f64x8::splat(8.0) * t190 * t731 + t1580));
            let tv3rhosigma25 = t6 * t1582 + t735;
            acc_v3rhosigma2_5 = tv3rhosigma25;
            let t1593 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t206 * t684 - t1490 + f64x8::splat(3.0) / f64x8::splat(4.0) * t206 * t687 + t1509 - f64x8::splat(3.0) / f64x8::splat(4.0) * t206 * t692 - t1524 + f64x8::splat(3.0) / f64x8::splat(8.0) * t206 * t701 + t1540));
            let tv3rhosigma26 = t6 * t1593 + t705;
            acc_v3rhosigma2_6 = tv3rhosigma26;
            let tv3rhosigma27 = f64x8::splat(0.0);
            acc_v3rhosigma2_7 = tv3rhosigma27;
            let tv3rhosigma28 = f64x8::splat(0.0);
            acc_v3rhosigma2_8 = tv3rhosigma28;
            let tv3rhosigma29 = f64x8::splat(0.0);
            acc_v3rhosigma2_9 = tv3rhosigma29;
            let tv3rhosigma210 = f64x8::splat(0.0);
            acc_v3rhosigma2_10 = tv3rhosigma210;
            let t1597 = t46 * t227;
            let t1600 = v_sigma2 * t232;
            let t1603 = -t44 * t1597 / f64x8::splat(54.0) - t58 * t1600 / f64x8::splat(48.0);
            let t1604 = t27 * t1603;
            let t1605 = t1604 * t125;
            let t1608 = t713 * t250;
            let t1613 = t643 * t306;
            let t1616 = t295 * t537;
            let t1617 = t1616 * t658;
            let t1620 = t296 * t669;
            let t1625 = t237 * t721;
            let t1628 = t720 * t249;
            let t1629 = t1475 * t1628;
            let t1632 = t305 * t668;
            let t1633 = t657 * t1632;
            let t1638 = t237 * t730;
            let t1641 = t729 * t249;
            let t1642 = t657 * t1641;
            let t1649 = -t70 * t1597 / f64x8::splat(54.0) - t73 * t1600 / f64x8::splat(48.0);
            let t1650 = t242 * t1649;
            let t1651 = t117 * t1650;
            let t1654 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t216 * t714 - t1565 - f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t1605 + f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t1608 + f64x8::splat(3.0) / f64x8::splat(4.0) * t216 * t717 + t1570 + f64x8::splat(3.0) / f64x8::splat(4.0) * t91 * t1613 - f64x8::splat(3.0) / f64x8::splat(2.0) * t656 * t1617 + f64x8::splat(3.0) / f64x8::splat(4.0) * t91 * t1620 - f64x8::splat(3.0) / f64x8::splat(4.0) * t216 * t722 - t1575 - f64x8::splat(3.0) / f64x8::splat(4.0) * t91 * t1625 + f64x8::splat(9.0) / f64x8::splat(4.0) * t656 * t1629 - f64x8::splat(3.0) / f64x8::splat(2.0) * t656 * t1633 + f64x8::splat(3.0) / f64x8::splat(8.0) * t216 * t731 + t1580 + f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t1638 - f64x8::splat(3.0) / f64x8::splat(4.0) * t656 * t1642 + f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t1651;
            let t1655 = ((t82).select(f64x8::splat(0.0), t1654));
            let tv3rhosigma211 = t6 * t1655 + t735;
            acc_v3rhosigma2_11 = tv3rhosigma211;
            let t1659 = t2 / t3 / t56;
            let t1660 = t1659 * t25;
            let t1661 = t27 * param_a3;
            let t1662 = t61 * t77;
            let t1663 = t1661 * t1662;
            let t1666 = t683 * t279;
            let t1669 = t269 * t691;
            let t1672 = t269 * t700;
            let t1675 = t690 * t278;
            let t1676 = t808 * t1675;
            let t1677 = t66 * t1676;
            let t1680 = t278 * t699;
            let t1681 = t591 * t1680;
            let t1684 = t1659 * t589;
            let t1685 = t65 * t170;
            let t1686 = param_b3 * t61;
            let t1687 = t1685 * t1686;
            let t1691 = ((t1).select(f64x8::splat(0.0), -t1660 * t1663 / f64x8::splat(1024.0) + f64x8::splat(9.0) / f64x8::splat(8.0) * t26 * t1666 - f64x8::splat(9.0) / f64x8::splat(4.0) * t26 * t1669 + f64x8::splat(9.0) / f64x8::splat(8.0) * t26 * t1672 + f64x8::splat(9.0) / f64x8::splat(4.0) * t26 * t1677 - f64x8::splat(9.0) / f64x8::splat(4.0) * t590 * t1681 + t1684 * t1687 / f64x8::splat(1024.0)));
            let tv3sigma30 = t6 * t1691;
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
            let t1692 = t1659 * t90;
            let t1693 = t112 * t125;
            let t1694 = t1661 * t1693;
            let t1697 = t713 * t306;
            let t1700 = t296 * t721;
            let t1703 = t296 * t730;
            let t1706 = t720 * t305;
            let t1707 = t1104 * t1706;
            let t1708 = t117 * t1707;
            let t1711 = t305 * t729;
            let t1712 = t657 * t1711;
            let t1715 = t1659 * t655;
            let t1716 = t116 * t242;
            let t1717 = param_b3 * t112;
            let t1718 = t1716 * t1717;
            let t1722 = ((t82).select(f64x8::splat(0.0), -t1692 * t1694 / f64x8::splat(1024.0) + f64x8::splat(9.0) / f64x8::splat(8.0) * t91 * t1697 - f64x8::splat(9.0) / f64x8::splat(4.0) * t91 * t1700 + f64x8::splat(9.0) / f64x8::splat(8.0) * t91 * t1703 + f64x8::splat(9.0) / f64x8::splat(4.0) * t91 * t1708 - f64x8::splat(9.0) / f64x8::splat(4.0) * t656 * t1712 + t1715 * t1718 / f64x8::splat(1024.0)));
            let tv3sigma39 = t6 * t1722;
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
