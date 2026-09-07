//! GGA_X_Q1D kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_q1d.c`
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
pub fn gga_x_q1d_kxc_pol(
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
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
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
            let t26 = (simd::cbrt(t6));
            let t27 = t25 * t26;
            let t28 = f64x8::splat(M_CBRT6);
            let t29 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t30 = (simd::cbrt(t29));
            let t31 = t30 * t30;
            let t32 = f64x8::splat(1.0) / t31;
            let t33 = t28 * t32;
            let t34 = v_rho0 * v_rho0;
            let t35 = (simd::cbrt(v_rho0));
            let t36 = t35 * t35;
            let t38 = f64x8::splat(1.0) / t36 / t34;
            let t40 = t33 * v_sigma0 * t38;
            let t42 = f64x8::splat(0.804) + f64x8::splat(5.0) / f64x8::splat(972.0) * t40;
            let t44 = f64x8::splat(0.646416) / t42;
            let t46 = t28 * t28;
            let t48 = f64x8::splat(1.0) / t30 / t29;
            let t49 = t46 * t48;
            let t50 = v_sigma0 * v_sigma0;
            let t51 = t34 * t34;
            let t52 = t51 * v_rho0;
            let t54 = f64x8::splat(1.0) / t35 / t52;
            let t57 = t49 * t50 * t54 / f64x8::splat(576.0);
            let t58 = t40 / f64x8::splat(24.0) + t57;
            let t59 = t29 * t29;
            let t60 = f64x8::splat(1.0) / t59;
            let t61 = t50 * v_sigma0;
            let t62 = t60 * t61;
            let t63 = t51 * t51;
            let t64 = f64x8::splat(1.0) / t63;
            let t67 = f64x8::splat(1.0) + t57 + t62 * t64 / f64x8::splat(2304.0);
            let t68 = f64x8::splat(1.0) / t67;
            let t69 = t58 * t68;
            let t71 = (f64x8::splat(1.804) - t44) * t28;
            let t72 = t32 * v_sigma0;
            let t76 = -t71 * t72 * t38 / f64x8::splat(24.0) + f64x8::splat(0.06525);
            let t78 = f64x8::splat(1.804) - t44 + t69 * t76;
            let t82 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t78));
            let t83 = (v_rho1).simd_le(dens_threshold);
            let t84 = -t16;
            let t86 = ((t14).select(t11, (t10).select(t15, t84 * t7)));
            let t87 = f64x8::splat(1.0) + t86;
            let t88 = (t87).simd_le(zeta_threshold);
            let t89 = (simd::cbrt(t87));
            let t91 = ((t88).select(t22, t89 * t87));
            let t92 = t91 * t26;
            let t93 = v_rho1 * v_rho1;
            let t94 = (simd::cbrt(v_rho1));
            let t95 = t94 * t94;
            let t97 = f64x8::splat(1.0) / t95 / t93;
            let t99 = t33 * v_sigma2 * t97;
            let t101 = f64x8::splat(0.804) + f64x8::splat(5.0) / f64x8::splat(972.0) * t99;
            let t103 = f64x8::splat(0.646416) / t101;
            let t105 = v_sigma2 * v_sigma2;
            let t106 = t93 * t93;
            let t107 = t106 * v_rho1;
            let t109 = f64x8::splat(1.0) / t94 / t107;
            let t112 = t49 * t105 * t109 / f64x8::splat(576.0);
            let t113 = t99 / f64x8::splat(24.0) + t112;
            let t114 = t105 * v_sigma2;
            let t115 = t60 * t114;
            let t116 = t106 * t106;
            let t117 = f64x8::splat(1.0) / t116;
            let t120 = f64x8::splat(1.0) + t112 + t115 * t117 / f64x8::splat(2304.0);
            let t121 = f64x8::splat(1.0) / t120;
            let t122 = t113 * t121;
            let t124 = (f64x8::splat(1.804) - t103) * t28;
            let t125 = t32 * v_sigma2;
            let t129 = -t124 * t125 * t97 / f64x8::splat(24.0) + f64x8::splat(0.06525);
            let t131 = f64x8::splat(1.804) - t103 + t122 * t129;
            let t135 = ((t83).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t92 * t131));
            let tzk0 = t82 + t135;
            acc_zk = tzk0;
            let t136 = t6 * t6;
            let t137 = f64x8::splat(1.0) / t136;
            let t138 = t16 * t137;
            let t140 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t138)));
            let t143 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t140));
            let t144 = t143 * t26;
            let t148 = t26 * t26;
            let t149 = f64x8::splat(1.0) / t148;
            let t150 = t25 * t149;
            let t153 = t5 * t150 * t78 / f64x8::splat(8.0);
            let t154 = t42 * t42;
            let t155 = f64x8::splat(1.0) / t154;
            let t156 = t155 * t28;
            let t157 = t34 * v_rho0;
            let t159 = f64x8::splat(1.0) / t36 / t157;
            let t160 = t72 * t159;
            let t166 = t51 * t34;
            let t168 = f64x8::splat(1.0) / t35 / t166;
            let t171 = t49 * t50 * t168 / f64x8::splat(108.0);
            let t172 = -t33 * v_sigma0 * t159 / f64x8::splat(9.0) - t171;
            let t173 = t172 * t68;
            let t175 = t67 * t67;
            let t176 = f64x8::splat(1.0) / t175;
            let t177 = t58 * t176;
            let t178 = t63 * v_rho0;
            let t179 = f64x8::splat(1.0) / t178;
            let t182 = -t171 - t62 * t179 / f64x8::splat(288.0);
            let t183 = t76 * t182;
            let t185 = t155 * t46;
            let t186 = t48 * t50;
            let t192 = f64x8::splat(0.0003694650205761317) * t185 * t186 * t168 + t71 * t160 / f64x8::splat(9.0);
            let t194 = -f64x8::splat(0.00886716049382716) * t156 * t160 + t173 * t76 - t177 * t183 + t69 * t192;
            let t199 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t144 * t78 - t153 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t194));
            let t200 = t84 * t137;
            let t202 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t200)));
            let t205 = ((t88).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t89 * t202));
            let t206 = t205 * t26;
            let t210 = t91 * t149;
            let t213 = t5 * t210 * t131 / f64x8::splat(8.0);
            let t215 = ((t83).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t206 * t131 - t213));
            let tvrho0 = t82 + t135 + t6 * (t199 + t215);
            acc_vrho_0 = tvrho0;
            let t219 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t138)));
            let t222 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t219));
            let t223 = t222 * t26;
            let t228 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t223 * t78 - t153));
            let t230 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t200)));
            let t233 = ((t88).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t89 * t230));
            let t234 = t233 * t26;
            let t238 = t101 * t101;
            let t239 = f64x8::splat(1.0) / t238;
            let t240 = t239 * t28;
            let t241 = t93 * v_rho1;
            let t243 = f64x8::splat(1.0) / t95 / t241;
            let t244 = t125 * t243;
            let t250 = t106 * t93;
            let t252 = f64x8::splat(1.0) / t94 / t250;
            let t255 = t49 * t105 * t252 / f64x8::splat(108.0);
            let t256 = -t33 * v_sigma2 * t243 / f64x8::splat(9.0) - t255;
            let t257 = t256 * t121;
            let t259 = t120 * t120;
            let t260 = f64x8::splat(1.0) / t259;
            let t261 = t113 * t260;
            let t262 = t116 * v_rho1;
            let t263 = f64x8::splat(1.0) / t262;
            let t266 = -t255 - t115 * t263 / f64x8::splat(288.0);
            let t267 = t129 * t266;
            let t269 = t239 * t46;
            let t270 = t48 * t105;
            let t276 = f64x8::splat(0.0003694650205761317) * t269 * t270 * t252 + t124 * t244 / f64x8::splat(9.0);
            let t278 = -f64x8::splat(0.00886716049382716) * t240 * t244 + t257 * t129 - t261 * t267 + t122 * t276;
            let t283 = ((t83).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t234 * t131 - t213 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t92 * t278));
            let tvrho1 = t82 + t135 + t6 * (t228 + t283);
            acc_vrho_1 = tvrho1;
            let t286 = t32 * t38;
            let t293 = t49 * v_sigma0 * t54 / f64x8::splat(288.0);
            let t294 = t33 * t38 / f64x8::splat(24.0) + t293;
            let t295 = t294 * t68;
            let t297 = t60 * t50;
            let t300 = t293 + t297 * t64 / f64x8::splat(768.0);
            let t301 = t76 * t300;
            let t303 = t48 * t54;
            let t309 = -f64x8::splat(0.00013854938271604938) * t185 * t303 * v_sigma0 - t71 * t286 / f64x8::splat(24.0);
            let t311 = f64x8::splat(0.0033251851851851854) * t156 * t286 + t295 * t76 - t177 * t301 + t69 * t309;
            let t315 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t311));
            let tvsigma0 = t6 * t315;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t316 = t32 * t97;
            let t323 = t49 * v_sigma2 * t109 / f64x8::splat(288.0);
            let t324 = t33 * t97 / f64x8::splat(24.0) + t323;
            let t325 = t324 * t121;
            let t327 = t60 * t105;
            let t330 = t323 + t327 * t117 / f64x8::splat(768.0);
            let t331 = t129 * t330;
            let t333 = t48 * t109;
            let t339 = -f64x8::splat(0.00013854938271604938) * t269 * t333 * v_sigma2 - t124 * t316 / f64x8::splat(24.0);
            let t341 = f64x8::splat(0.0033251851851851854) * t240 * t316 + t325 * t129 - t261 * t331 + t122 * t339;
            let t345 = ((t83).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t92 * t341));
            let tvsigma2 = t6 * t345;
            acc_vsigma_2 = tvsigma2;
            let t348 = t23 * t23;
            let t349 = f64x8::splat(1.0) / t348;
            let t350 = t140 * t140;
            let t353 = t136 * t6;
            let t354 = f64x8::splat(1.0) / t353;
            let t355 = t16 * t354;
            let t358 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t137 + f64x8::splat(2.0) * t355)));
            let t362 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t349 * t350 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t358));
            let t363 = t362 * t26;
            let t367 = t143 * t149;
            let t369 = t5 * t367 * t78;
            let t375 = f64x8::splat(1.0) / t148 / t6;
            let t376 = t25 * t375;
            let t379 = t5 * t376 * t78 / f64x8::splat(12.0);
            let t381 = t5 * t150 * t194;
            let t384 = f64x8::splat(1.0) / t154 / t42;
            let t385 = t384 * t46;
            let t386 = t51 * t157;
            let t388 = f64x8::splat(1.0) / t35 / t386;
            let t389 = t186 * t388;
            let t393 = f64x8::splat(1.0) / t36 / t51;
            let t394 = t72 * t393;
            let t402 = f64x8::splat(19.0) / f64x8::splat(324.0) * t49 * t50 * t388;
            let t403 = f64x8::splat(11.0) / f64x8::splat(27.0) * t33 * v_sigma0 * t393 + t402;
            let t404 = t403 * t68;
            let t406 = t172 * t176;
            let t412 = f64x8::splat(1.0) / t175 / t67;
            let t413 = t58 * t412;
            let t414 = t182 * t182;
            let t415 = t76 * t414;
            let t418 = t192 * t182;
            let t421 = t63 * t34;
            let t422 = f64x8::splat(1.0) / t421;
            let t425 = t402 + t62 * t422 / f64x8::splat(32.0);
            let t426 = t76 * t425;
            let t428 = t384 * t60;
            let t436 = f64x8::splat(6.081728733763484e-05) * t428 * t61 * t422 - f64x8::splat(0.0033251851851851854) * t185 * t389 - f64x8::splat(11.0) / f64x8::splat(27.0) * t71 * t394;
            let t438 = -f64x8::splat(0.00024326914935053937) * t385 * t389 + f64x8::splat(0.03251292181069959) * t156 * t394 + t404 * t76 - f64x8::splat(2.0) * t406 * t183 + f64x8::splat(2.0) * t173 * t192 + f64x8::splat(2.0) * t413 * t415 - f64x8::splat(2.0) * t177 * t418 - t177 * t426 + t69 * t436;
            let t443 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t363 * t78 - t369 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t144 * t194 + t379 - t381 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t438));
            let t444 = t89 * t89;
            let t445 = f64x8::splat(1.0) / t444;
            let t446 = t202 * t202;
            let t449 = t84 * t354;
            let t452 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t137 + f64x8::splat(2.0) * t449)));
            let t456 = ((t88).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t445 * t446 + f64x8::splat(4.0) / f64x8::splat(3.0) * t89 * t452));
            let t457 = t456 * t26;
            let t461 = t205 * t149;
            let t463 = t5 * t461 * t131;
            let t465 = t91 * t375;
            let t468 = t5 * t465 * t131 / f64x8::splat(12.0);
            let t470 = ((t83).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t457 * t131 - t463 / f64x8::splat(4.0) + t468));
            let tv2rho20 = f64x8::splat(2.0) * t199 + f64x8::splat(2.0) * t215 + t6 * (t443 + t470);
            acc_v2rho2_0 = tv2rho20;
            let t473 = t349 * t219;
            let t477 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t355)));
            let t481 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t473 * t140 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t477));
            let t482 = t481 * t26;
            let t486 = t222 * t149;
            let t488 = t5 * t486 * t78;
            let t496 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t482 * t78 - t488 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t223 * t194 - t369 / f64x8::splat(8.0) + t379 - t381 / f64x8::splat(8.0)));
            let t497 = t445 * t230;
            let t501 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t449)));
            let t505 = ((t88).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t497 * t202 + f64x8::splat(4.0) / f64x8::splat(3.0) * t89 * t501));
            let t506 = t505 * t26;
            let t510 = t233 * t149;
            let t512 = t5 * t510 * t131;
            let t519 = t5 * t210 * t278;
            let t522 = ((t83).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t506 * t131 - t512 / f64x8::splat(8.0) - t463 / f64x8::splat(8.0) + t468 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t206 * t278 - t519 / f64x8::splat(8.0)));
            let tv2rho21 = t199 + t215 + t228 + t283 + t6 * (t496 + t522);
            acc_v2rho2_1 = tv2rho21;
            let t527 = t219 * t219;
            let t532 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t137 + f64x8::splat(2.0) * t355)));
            let t536 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t349 * t527 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t532));
            let t537 = t536 * t26;
            let t543 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t537 * t78 - t488 / f64x8::splat(4.0) + t379));
            let t544 = t230 * t230;
            let t549 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t137 + f64x8::splat(2.0) * t449)));
            let t553 = ((t88).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t445 * t544 + f64x8::splat(4.0) / f64x8::splat(3.0) * t89 * t549));
            let t554 = t553 * t26;
            let t564 = f64x8::splat(1.0) / t238 / t101;
            let t565 = t564 * t46;
            let t566 = t106 * t241;
            let t568 = f64x8::splat(1.0) / t94 / t566;
            let t569 = t270 * t568;
            let t573 = f64x8::splat(1.0) / t95 / t106;
            let t574 = t125 * t573;
            let t582 = f64x8::splat(19.0) / f64x8::splat(324.0) * t49 * t105 * t568;
            let t583 = f64x8::splat(11.0) / f64x8::splat(27.0) * t33 * v_sigma2 * t573 + t582;
            let t584 = t583 * t121;
            let t586 = t256 * t260;
            let t592 = f64x8::splat(1.0) / t259 / t120;
            let t593 = t113 * t592;
            let t594 = t266 * t266;
            let t595 = t129 * t594;
            let t598 = t276 * t266;
            let t601 = t116 * t93;
            let t602 = f64x8::splat(1.0) / t601;
            let t605 = t582 + t115 * t602 / f64x8::splat(32.0);
            let t606 = t129 * t605;
            let t608 = t564 * t60;
            let t616 = f64x8::splat(6.081728733763484e-05) * t608 * t114 * t602 - f64x8::splat(0.0033251851851851854) * t269 * t569 - f64x8::splat(11.0) / f64x8::splat(27.0) * t124 * t574;
            let t618 = -f64x8::splat(0.00024326914935053937) * t565 * t569 + f64x8::splat(0.03251292181069959) * t240 * t574 + t584 * t129 - f64x8::splat(2.0) * t586 * t267 + f64x8::splat(2.0) * t257 * t276 + f64x8::splat(2.0) * t593 * t595 - f64x8::splat(2.0) * t261 * t598 - t261 * t606 + t122 * t616;
            let t623 = ((t83).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t554 * t131 - t512 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t234 * t278 + t468 - t519 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t92 * t618));
            let tv2rho22 = f64x8::splat(2.0) * t228 + f64x8::splat(2.0) * t283 + t6 * (t543 + t623);
            acc_v2rho2_2 = tv2rho22;
            let t631 = t5 * t150 * t311 / f64x8::splat(8.0);
            let t632 = t48 * t168;
            let t633 = t632 * v_sigma0;
            let t636 = t32 * t159;
            let t643 = t49 * v_sigma0 * t168 / f64x8::splat(54.0);
            let t644 = -t33 * t159 / f64x8::splat(9.0) - t643;
            let t645 = t644 * t68;
            let t647 = t294 * t176;
            let t651 = t301 * t182;
            let t654 = t192 * t300;
            let t658 = -t643 - t297 * t179 / f64x8::splat(96.0);
            let t659 = t76 * t658;
            let t662 = t309 * t182;
            let t671 = -f64x8::splat(2.2806482751613066e-05) * t428 * t179 * t50 + f64x8::splat(0.001108395061728395) * t185 * t633 + t71 * t636 / f64x8::splat(9.0);
            let t673 = f64x8::splat(9.122593100645226e-05) * t385 * t633 - f64x8::splat(0.00886716049382716) * t156 * t636 + t645 * t76 - t647 * t183 + t295 * t192 - t406 * t301 + f64x8::splat(2.0) * t413 * t651 - t177 * t654 - t177 * t659 + t173 * t309 - t177 * t662 + t69 * t671;
            let t678 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t144 * t311 - t631 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t673));
            let tv2rhosigma0 = t6 * t678 + t315;
            acc_v2rhosigma_0 = tv2rhosigma0;
            let tv2rhosigma1 = f64x8::splat(0.0);
            acc_v2rhosigma_1 = tv2rhosigma1;
            let t685 = t5 * t210 * t341 / f64x8::splat(8.0);
            let t687 = ((t83).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t206 * t341 - t685));
            let tv2rhosigma2 = t6 * t687 + t345;
            acc_v2rhosigma_2 = tv2rhosigma2;
            let t693 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t223 * t311 - t631));
            let tv2rhosigma3 = t6 * t693 + t315;
            acc_v2rhosigma_3 = tv2rhosigma3;
            let tv2rhosigma4 = f64x8::splat(0.0);
            acc_v2rhosigma_4 = tv2rhosigma4;
            let t698 = t48 * t252;
            let t699 = t698 * v_sigma2;
            let t702 = t32 * t243;
            let t709 = t49 * v_sigma2 * t252 / f64x8::splat(54.0);
            let t710 = -t33 * t243 / f64x8::splat(9.0) - t709;
            let t711 = t710 * t121;
            let t713 = t324 * t260;
            let t717 = t331 * t266;
            let t720 = t276 * t330;
            let t724 = -t709 - t327 * t263 / f64x8::splat(96.0);
            let t725 = t129 * t724;
            let t728 = t339 * t266;
            let t737 = -f64x8::splat(2.2806482751613066e-05) * t608 * t263 * t105 + f64x8::splat(0.001108395061728395) * t269 * t699 + t124 * t702 / f64x8::splat(9.0);
            let t739 = f64x8::splat(9.122593100645226e-05) * t565 * t699 - f64x8::splat(0.00886716049382716) * t240 * t702 + t711 * t129 - t713 * t267 + t325 * t276 - t586 * t331 + f64x8::splat(2.0) * t593 * t717 - t261 * t720 - t261 * t725 + t257 * t339 - t261 * t728 + t122 * t737;
            let t744 = ((t83).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t234 * t341 - t685 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t92 * t739));
            let tv2rhosigma5 = t6 * t744 + t345;
            acc_v2rhosigma_5 = tv2rhosigma5;
            let t748 = t54 * t68;
            let t756 = t300 * t300;
            let t757 = t76 * t756;
            let t760 = t309 * t300;
            let t763 = t49 * t54;
            let t765 = t60 * v_sigma0;
            let t768 = t763 / f64x8::splat(288.0) + t765 * t64 / f64x8::splat(384.0);
            let t769 = t76 * t768;
            let t776 = f64x8::splat(8.5524310318549e-06) * t428 * t64 * v_sigma0 - f64x8::splat(0.00027709876543209876) * t185 * t303;
            let t778 = -f64x8::splat(3.42097241274196e-05) * t385 * t303 + t49 * t748 * t76 / f64x8::splat(288.0) - f64x8::splat(2.0) * t647 * t301 + f64x8::splat(2.0) * t295 * t309 + f64x8::splat(2.0) * t413 * t757 - f64x8::splat(2.0) * t177 * t760 - t177 * t769 + t69 * t776;
            let t782 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t778));
            let tv2sigma20 = t6 * t782;
            acc_v2sigma2_0 = tv2sigma20;
            let tv2sigma21 = f64x8::splat(0.0);
            acc_v2sigma2_1 = tv2sigma21;
            let tv2sigma22 = f64x8::splat(0.0);
            acc_v2sigma2_2 = tv2sigma22;
            let tv2sigma23 = f64x8::splat(0.0);
            acc_v2sigma2_3 = tv2sigma23;
            let tv2sigma24 = f64x8::splat(0.0);
            acc_v2sigma2_4 = tv2sigma24;
            let t785 = t109 * t121;
            let t793 = t330 * t330;
            let t794 = t129 * t793;
            let t797 = t339 * t330;
            let t800 = t49 * t109;
            let t802 = t60 * v_sigma2;
            let t805 = t800 / f64x8::splat(288.0) + t802 * t117 / f64x8::splat(384.0);
            let t806 = t129 * t805;
            let t813 = f64x8::splat(8.5524310318549e-06) * t608 * t117 * v_sigma2 - f64x8::splat(0.00027709876543209876) * t269 * t333;
            let t815 = -f64x8::splat(3.42097241274196e-05) * t565 * t333 + t49 * t785 * t129 / f64x8::splat(288.0) - f64x8::splat(2.0) * t713 * t331 + f64x8::splat(2.0) * t325 * t339 + f64x8::splat(2.0) * t593 * t794 - f64x8::splat(2.0) * t261 * t797 - t261 * t806 + t122 * t813;
            let t819 = ((t83).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t92 * t815));
            let tv2sigma25 = t6 * t819;
            acc_v2sigma2_5 = tv2sigma25;
            let t823 = f64x8::splat(1.0) / t348 / t19;
            let t824 = t350 * t140;
            let t827 = t349 * t140;
            let t830 = t136 * t136;
            let t831 = f64x8::splat(1.0) / t830;
            let t832 = t16 * t831;
            let t835 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(6.0) * t354 - f64x8::splat(6.0) * t832)));
            let t839 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t823 * t824 + f64x8::splat(4.0) / f64x8::splat(3.0) * t827 * t358 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t835));
            let t840 = t839 * t26;
            let t844 = t362 * t149;
            let t846 = t5 * t844 * t78;
            let t851 = t143 * t375;
            let t853 = t5 * t851 * t78;
            let t856 = t5 * t367 * t194;
            let t862 = f64x8::splat(1.0) / t148 / t136;
            let t863 = t25 * t862;
            let t866 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t863 * t78;
            let t868 = t5 * t376 * t194;
            let t871 = t5 * t150 * t438;
            let t873 = t175 * t175;
            let t874 = f64x8::splat(1.0) / t873;
            let t875 = t58 * t874;
            let t876 = t414 * t182;
            let t877 = t76 * t876;
            let t880 = t183 * t425;
            let t883 = t172 * t412;
            let t887 = f64x8::splat(1.0) / t35 / t63;
            let t888 = t186 * t887;
            let t892 = f64x8::splat(1.0) / t36 / t52;
            let t893 = t72 * t892;
            let t896 = t436 * t182;
            let t899 = t192 * t425;
            let t904 = f64x8::splat(209.0) / f64x8::splat(486.0) * t49 * t50 * t887;
            let t905 = t63 * t157;
            let t906 = f64x8::splat(1.0) / t905;
            let t909 = -t904 - f64x8::splat(5.0) / f64x8::splat(16.0) * t62 * t906;
            let t910 = t76 * t909;
            let t912 = t154 * t154;
            let t914 = f64x8::splat(1.0) / t912 * t60;
            let t915 = t61 * t906;
            let t918 = t403 * t176;
            let t925 = t192 * t414;
            let t931 = -f64x8::splat(154.0) / f64x8::splat(81.0) * t33 * v_sigma0 * t892 - t904;
            let t932 = t931 * t68;
            let t938 = t50 * t50;
            let t939 = t914 * t938;
            let t940 = t63 * t52;
            let t942 = f64x8::splat(1.0) / t36 / t940;
            let t953 = f64x8::splat(2.502769026240117e-06) * t939 * t942 * t28 * t32 - f64x8::splat(0.001155528459415062) * t428 * t915 + f64x8::splat(0.0279972382258802) * t185 * t888 + f64x8::splat(154.0) / f64x8::splat(81.0) * t71 * t893;
            let t955 = -f64x8::splat(6.0) * t875 * t877 + f64x8::splat(6.0) * t413 * t880 + f64x8::splat(6.0) * t883 * t415 + f64x8::splat(0.002675960642855933) * t385 * t888 - f64x8::splat(0.15172696844993142) * t156 * t893 - f64x8::splat(3.0) * t177 * t896 - f64x8::splat(3.0) * t177 * t899 - t177 * t910 - f64x8::splat(6.006645662976281e-05) * t914 * t915 - f64x8::splat(3.0) * t918 * t183 - f64x8::splat(6.0) * t406 * t418 - f64x8::splat(3.0) * t406 * t426 + f64x8::splat(6.0) * t413 * t925 + t932 * t76 + f64x8::splat(3.0) * t404 * t192 + f64x8::splat(3.0) * t173 * t436 + t69 * t953;
            let t960 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t840 * t78 - f64x8::splat(3.0) / f64x8::splat(8.0) * t846 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t363 * t194 + t853 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t856 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t144 * t438 - t866 + t868 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t871 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t955));
            let t962 = f64x8::splat(1.0) / t444 / t87;
            let t963 = t446 * t202;
            let t966 = t445 * t202;
            let t969 = t84 * t831;
            let t972 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t354 - f64x8::splat(6.0) * t969)));
            let t976 = ((t88).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t962 * t963 + f64x8::splat(4.0) / f64x8::splat(3.0) * t966 * t452 + f64x8::splat(4.0) / f64x8::splat(3.0) * t89 * t972));
            let t977 = t976 * t26;
            let t981 = t456 * t149;
            let t983 = t5 * t981 * t131;
            let t985 = t205 * t375;
            let t987 = t5 * t985 * t131;
            let t989 = t91 * t862;
            let t992 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t989 * t131;
            let t994 = ((t83).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t977 * t131 - f64x8::splat(3.0) / f64x8::splat(8.0) * t983 + t987 / f64x8::splat(4.0) - t992));
            let tv3rho30 = f64x8::splat(3.0) * t443 + f64x8::splat(3.0) * t470 + t6 * (t960 + t994);
            acc_v3rho3_0 = tv3rho30;
            let t997 = f64x8::splat(2.0) * t496;
            let t998 = f64x8::splat(2.0) * t522;
            let t999 = t823 * t219;
            let t1002 = t349 * t477;
            let t1007 = f64x8::splat(2.0) * t354;
            let t1008 = f64x8::splat(6.0) * t832;
            let t1010 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t1007 - t1008)));
            let t1014 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t999 * t350 + f64x8::splat(8.0) / f64x8::splat(9.0) * t1002 * t140 + f64x8::splat(4.0) / f64x8::splat(9.0) * t473 * t358 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t1010));
            let t1015 = t1014 * t26;
            let t1019 = t481 * t149;
            let t1022 = t5 * t1019 * t78 / f64x8::splat(4.0);
            let t1026 = t222 * t375;
            let t1028 = t5 * t1026 * t78;
            let t1032 = t5 * t486 * t194 / f64x8::splat(4.0);
            let t1041 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t1015 * t78 - t1022 - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t482 * t194 + t1028 / f64x8::splat(12.0) - t1032 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t223 * t438 - t846 / f64x8::splat(8.0) + t853 / f64x8::splat(6.0) - t856 / f64x8::splat(4.0) - t866 + t868 / f64x8::splat(6.0) - t871 / f64x8::splat(8.0);
            let t1042 = ((t1).select(f64x8::splat(0.0), t1041));
            let t1043 = t962 * t230;
            let t1046 = t445 * t501;
            let t1051 = f64x8::splat(6.0) * t969;
            let t1053 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t1007 - t1051)));
            let t1057 = ((t88).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t1043 * t446 + f64x8::splat(8.0) / f64x8::splat(9.0) * t1046 * t202 + f64x8::splat(4.0) / f64x8::splat(9.0) * t497 * t452 + f64x8::splat(4.0) / f64x8::splat(3.0) * t89 * t1053));
            let t1058 = t1057 * t26;
            let t1062 = t505 * t149;
            let t1065 = t5 * t1062 * t131 / f64x8::splat(4.0);
            let t1066 = t233 * t375;
            let t1068 = t5 * t1066 * t131;
            let t1077 = t5 * t461 * t278 / f64x8::splat(4.0);
            let t1079 = t5 * t465 * t278;
            let t1082 = ((t83).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t1058 * t131 - t1065 + t1068 / f64x8::splat(12.0) - t983 / f64x8::splat(8.0) + t987 / f64x8::splat(6.0) - t992 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t457 * t278 - t1077 + t1079 / f64x8::splat(12.0)));
            let tv3rho31 = t443 + t470 + t997 + t998 + t6 * (t1042 + t1082);
            acc_v3rho3_1 = tv3rho31;
            let t1085 = t823 * t527;
            let t1090 = t349 * t532;
            let t1094 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t1007 - t1008)));
            let t1098 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t1085 * t140 + f64x8::splat(8.0) / f64x8::splat(9.0) * t473 * t477 + f64x8::splat(4.0) / f64x8::splat(9.0) * t1090 * t140 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t1094));
            let t1099 = t1098 * t26;
            let t1103 = t536 * t149;
            let t1105 = t5 * t1103 * t78;
            let t1114 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t1099 * t78 - t1105 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t537 * t194 - t1022 + t1028 / f64x8::splat(6.0) - t1032 + t853 / f64x8::splat(12.0) - t866 + t868 / f64x8::splat(12.0)));
            let t1115 = t962 * t544;
            let t1120 = t445 * t549;
            let t1124 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t1007 - t1051)));
            let t1128 = ((t88).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t1115 * t202 + f64x8::splat(8.0) / f64x8::splat(9.0) * t497 * t501 + f64x8::splat(4.0) / f64x8::splat(9.0) * t1120 * t202 + f64x8::splat(4.0) / f64x8::splat(3.0) * t89 * t1124));
            let t1129 = t1128 * t26;
            let t1133 = t553 * t149;
            let t1135 = t5 * t1133 * t131;
            let t1142 = t5 * t510 * t278;
            let t1150 = t5 * t210 * t618;
            let t1152 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t1129 * t131 - t1135 / f64x8::splat(8.0) - t1065 + t1068 / f64x8::splat(6.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t506 * t278 - t1142 / f64x8::splat(4.0) + t987 / f64x8::splat(12.0) - t992 - t1077 + t1079 / f64x8::splat(6.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t206 * t618 - t1150 / f64x8::splat(8.0);
            let t1153 = ((t83).select(f64x8::splat(0.0), t1152));
            let tv3rho32 = t997 + t998 + t543 + t623 + t6 * (t1114 + t1153);
            acc_v3rho3_2 = tv3rho32;
            let t1158 = t527 * t219;
            let t1165 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t354 - f64x8::splat(6.0) * t832)));
            let t1169 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t823 * t1158 + f64x8::splat(4.0) / f64x8::splat(3.0) * t473 * t532 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t1165));
            let t1170 = t1169 * t26;
            let t1177 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t1170 * t78 - f64x8::splat(3.0) / f64x8::splat(8.0) * t1105 + t1028 / f64x8::splat(4.0) - t866));
            let t1178 = t544 * t230;
            let t1185 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(6.0) * t354 - f64x8::splat(6.0) * t969)));
            let t1189 = ((t88).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t962 * t1178 + f64x8::splat(4.0) / f64x8::splat(3.0) * t497 * t549 + f64x8::splat(4.0) / f64x8::splat(3.0) * t89 * t1185));
            let t1190 = t1189 * t26;
            let t1205 = t256 * t592;
            let t1208 = t259 * t259;
            let t1209 = f64x8::splat(1.0) / t1208;
            let t1210 = t113 * t1209;
            let t1211 = t594 * t266;
            let t1212 = t129 * t1211;
            let t1215 = t267 * t605;
            let t1219 = f64x8::splat(1.0) / t95 / t107;
            let t1224 = f64x8::splat(1.0) / t94 / t116;
            let t1227 = f64x8::splat(209.0) / f64x8::splat(486.0) * t49 * t105 * t1224;
            let t1228 = -f64x8::splat(154.0) / f64x8::splat(81.0) * t33 * v_sigma2 * t1219 - t1227;
            let t1229 = t1228 * t121;
            let t1235 = t238 * t238;
            let t1237 = f64x8::splat(1.0) / t1235 * t60;
            let t1238 = t105 * t105;
            let t1239 = t1237 * t1238;
            let t1240 = t116 * t107;
            let t1242 = f64x8::splat(1.0) / t95 / t1240;
            let t1247 = t116 * t241;
            let t1248 = f64x8::splat(1.0) / t1247;
            let t1249 = t114 * t1248;
            let t1252 = t270 * t1224;
            let t1255 = t125 * t1219;
            let t1258 = f64x8::splat(2.502769026240117e-06) * t1239 * t1242 * t28 * t32 - f64x8::splat(0.001155528459415062) * t608 * t1249 + f64x8::splat(0.0279972382258802) * t269 * t1252 + f64x8::splat(154.0) / f64x8::splat(81.0) * t124 * t1255;
            let t1264 = t276 * t594;
            let t1267 = t616 * t266;
            let t1270 = t276 * t605;
            let t1275 = -t1227 - f64x8::splat(5.0) / f64x8::splat(16.0) * t115 * t1248;
            let t1276 = t129 * t1275;
            let t1280 = t583 * t260;
            let t1287 = f64x8::splat(6.0) * t1205 * t595 - f64x8::splat(6.0) * t1210 * t1212 + f64x8::splat(6.0) * t593 * t1215 + t1229 * t129 + f64x8::splat(3.0) * t584 * t276 + f64x8::splat(3.0) * t257 * t616 + t122 * t1258 + f64x8::splat(0.002675960642855933) * t565 * t1252 - f64x8::splat(0.15172696844993142) * t240 * t1255 + f64x8::splat(6.0) * t593 * t1264 - f64x8::splat(3.0) * t261 * t1267 - f64x8::splat(3.0) * t261 * t1270 - t261 * t1276 - f64x8::splat(6.006645662976281e-05) * t1237 * t1249 - f64x8::splat(3.0) * t1280 * t267 - f64x8::splat(6.0) * t586 * t598 - f64x8::splat(3.0) * t586 * t606;
            let t1292 = ((t83).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t1190 * t131 - f64x8::splat(3.0) / f64x8::splat(8.0) * t1135 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t554 * t278 + t1068 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t1142 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t234 * t618 - t992 + t1079 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t1150 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t92 * t1287));
            let tv3rho33 = f64x8::splat(3.0) * t543 + f64x8::splat(3.0) * t623 + t6 * (t1177 + t1292);
            acc_v3rho3_3 = tv3rho33;
            let t1300 = t5 * t367 * t311;
            let t1307 = t5 * t376 * t311 / f64x8::splat(12.0);
            let t1309 = t5 * t150 * t673;
            let t1311 = t301 * t414;
            let t1314 = t48 * t388;
            let t1315 = t1314 * v_sigma0;
            let t1318 = t654 * t182;
            let t1321 = t659 * t182;
            let t1324 = t301 * t425;
            let t1332 = t63 * t51;
            let t1334 = f64x8::splat(1.0) / t36 / t1332;
            let t1335 = t914 * t1334;
            let t1337 = t61 * t28 * t32;
            let t1340 = t422 * t50;
            let t1345 = t32 * t393;
            let t1348 = -f64x8::splat(9.385383848400439e-07) * t1335 * t1337 + f64x8::splat(0.00038771020677742216) * t428 * t1340 - f64x8::splat(0.008005075445816186) * t185 * t1315 - f64x8::splat(11.0) / f64x8::splat(27.0) * t71 * t1345;
            let t1354 = f64x8::splat(19.0) / f64x8::splat(162.0) * t49 * v_sigma0 * t388;
            let t1355 = f64x8::splat(11.0) / f64x8::splat(27.0) * t33 * t393 + t1354;
            let t1356 = t1355 * t68;
            let t1363 = t1354 + f64x8::splat(3.0) / f64x8::splat(32.0) * t297 * t422;
            let t1364 = t76 * t1363;
            let t1368 = -f64x8::splat(6.0) * t875 * t1311 - f64x8::splat(0.0008210333790580704) * t385 * t1315 + f64x8::splat(4.0) * t413 * t1318 + f64x8::splat(4.0) * t413 * t1321 + f64x8::splat(2.0) * t413 * t1324 + f64x8::splat(4.0) * t883 * t651 + t404 * t309 + f64x8::splat(2.0) * t173 * t671 + t69 * t1348 + t1356 * t76 + f64x8::splat(2.0) * t645 * t192 + t295 * t436 - t177 * t1364 - f64x8::splat(2.0) * t406 * t662;
            let t1369 = t671 * t182;
            let t1372 = t309 * t425;
            let t1376 = t644 * t176;
            let t1387 = t436 * t300;
            let t1389 = t192 * t658;
            let t1392 = t309 * t414;
            let t1397 = t294 * t412;
            let t1400 = -f64x8::splat(2.0) * t177 * t1369 - t177 * t1372 + f64x8::splat(2.2524921236161055e-05) * t914 * t1340 - f64x8::splat(2.0) * t1376 * t183 - f64x8::splat(2.0) * t647 * t418 - t647 * t426 - t918 * t301 - f64x8::splat(2.0) * t406 * t654 - f64x8::splat(2.0) * t406 * t659 - t177 * t1387 - f64x8::splat(2.0) * t177 * t1389 + f64x8::splat(2.0) * t413 * t1392 + f64x8::splat(0.03251292181069959) * t156 * t1345 + f64x8::splat(2.0) * t1397 * t415;
            let t1401 = t1368 + t1400;
            let t1406 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t363 * t311 - t1300 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t144 * t673 + t1307 - t1309 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t1401));
            let tv3rho2sigma0 = t6 * t1406 + f64x8::splat(2.0) * t678;
            acc_v3rho2sigma_0 = tv3rho2sigma0;
            let tv3rho2sigma1 = f64x8::splat(0.0);
            acc_v3rho2sigma_1 = tv3rho2sigma1;
            let t1413 = t5 * t461 * t341;
            let t1417 = t5 * t465 * t341 / f64x8::splat(12.0);
            let t1419 = ((t83).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t457 * t341 - t1413 / f64x8::splat(4.0) + t1417));
            let tv3rho2sigma2 = t6 * t1419 + f64x8::splat(2.0) * t687;
            acc_v3rho2sigma_2 = tv3rho2sigma2;
            let t1425 = t5 * t486 * t311;
            let t1433 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t482 * t311 - t1425 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t223 * t673 - t1300 / f64x8::splat(8.0) + t1307 - t1309 / f64x8::splat(8.0)));
            let tv3rho2sigma3 = t6 * t1433 + t678 + t693;
            acc_v3rho2sigma_3 = tv3rho2sigma3;
            let tv3rho2sigma4 = f64x8::splat(0.0);
            acc_v3rho2sigma_4 = tv3rho2sigma4;
            let t1439 = t5 * t510 * t341;
            let t1446 = t5 * t210 * t739;
            let t1449 = ((t83).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t506 * t341 - t1439 / f64x8::splat(8.0) - t1413 / f64x8::splat(8.0) + t1417 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t206 * t739 - t1446 / f64x8::splat(8.0)));
            let tv3rho2sigma5 = t6 * t1449 + t687 + t744;
            acc_v3rho2sigma_5 = tv3rho2sigma5;
            let t1457 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t537 * t311 - t1425 / f64x8::splat(4.0) + t1307));
            let tv3rho2sigma6 = t6 * t1457 + f64x8::splat(2.0) * t693;
            acc_v3rho2sigma_6 = tv3rho2sigma6;
            let tv3rho2sigma7 = f64x8::splat(0.0);
            acc_v3rho2sigma_7 = tv3rho2sigma7;
            let t1472 = t616 * t330;
            let t1474 = t276 * t724;
            let t1477 = t339 * t605;
            let t1479 = t602 * t105;
            let t1482 = t710 * t260;
            let t1489 = t720 * t266;
            let t1492 = t725 * t266;
            let t1495 = t331 * t605;
            let t1498 = t331 * t594;
            let t1501 = -f64x8::splat(2.0) * t586 * t720 - f64x8::splat(2.0) * t586 * t725 - t261 * t1472 - f64x8::splat(2.0) * t261 * t1474 - t261 * t1477 + f64x8::splat(2.2524921236161055e-05) * t1237 * t1479 - f64x8::splat(2.0) * t1482 * t267 - f64x8::splat(2.0) * t713 * t598 - t713 * t606 - t1280 * t331 + f64x8::splat(4.0) * t593 * t1489 + f64x8::splat(4.0) * t593 * t1492 + f64x8::splat(2.0) * t593 * t1495 - f64x8::splat(6.0) * t1210 * t1498;
            let t1502 = t48 * t568;
            let t1503 = t1502 * v_sigma2;
            let t1508 = t32 * t573;
            let t1513 = f64x8::splat(19.0) / f64x8::splat(162.0) * t49 * v_sigma2 * t568;
            let t1516 = t1513 + f64x8::splat(3.0) / f64x8::splat(32.0) * t327 * t602;
            let t1517 = t129 * t1516;
            let t1521 = t737 * t266;
            let t1524 = t339 * t594;
            let t1527 = t324 * t592;
            let t1534 = t116 * t106;
            let t1536 = f64x8::splat(1.0) / t95 / t1534;
            let t1537 = t1237 * t1536;
            let t1539 = t114 * t28 * t32;
            let t1548 = -f64x8::splat(9.385383848400439e-07) * t1537 * t1539 + f64x8::splat(0.00038771020677742216) * t608 * t1479 - f64x8::splat(0.008005075445816186) * t269 * t1503 - f64x8::splat(11.0) / f64x8::splat(27.0) * t124 * t1508;
            let t1552 = f64x8::splat(11.0) / f64x8::splat(27.0) * t33 * t573 + t1513;
            let t1553 = t1552 * t121;
            let t1557 = -f64x8::splat(0.0008210333790580704) * t565 * t1503 + f64x8::splat(4.0) * t1205 * t717 + f64x8::splat(0.03251292181069959) * t240 * t1508 - t261 * t1517 - f64x8::splat(2.0) * t586 * t728 - f64x8::splat(2.0) * t261 * t1521 + f64x8::splat(2.0) * t593 * t1524 + f64x8::splat(2.0) * t1527 * t595 + t325 * t616 + t584 * t339 + f64x8::splat(2.0) * t257 * t737 + t122 * t1548 + t1553 * t129 + f64x8::splat(2.0) * t711 * t276;
            let t1558 = t1501 + t1557;
            let t1563 = ((t83).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t554 * t341 - t1439 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t234 * t739 + t1417 - t1446 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t92 * t1558));
            let tv3rho2sigma8 = t6 * t1563 + f64x8::splat(2.0) * t744;
            acc_v3rho2sigma_8 = tv3rho2sigma8;
            let t1570 = t5 * t150 * t778 / f64x8::splat(8.0);
            let t1577 = t309 * t658;
            let t1581 = t192 * t768;
            let t1583 = t49 * t168;
            let t1587 = -t1583 / f64x8::splat(54.0) - t765 * t179 / f64x8::splat(48.0);
            let t1588 = t76 * t1587;
            let t1590 = t776 * t182;
            let t1592 = t179 * v_sigma0;
            let t1599 = t192 * t756;
            let t1604 = -f64x8::splat(2.0) * t647 * t654 - f64x8::splat(2.0) * t647 * t659 - f64x8::splat(2.0) * t647 * t662 - f64x8::splat(2.0) * t177 * t1577 - t406 * t769 - t177 * t1581 - t177 * t1588 - t177 * t1590 - f64x8::splat(8.446845463560395e-06) * t914 * t1592 - f64x8::splat(2.0) * t1376 * t301 + f64x8::splat(2.0) * t883 * t757 + f64x8::splat(2.0) * t413 * t1599 - f64x8::splat(2.0) * t406 * t760;
            let t1605 = t671 * t300;
            let t1612 = t757 * t182;
            let t1615 = t301 * t658;
            let t1618 = t760 * t182;
            let t1621 = t769 * t182;
            let t1624 = t168 * t68;
            let t1631 = t176 * t76;
            let t1632 = t1631 * t182;
            let t1636 = f64x8::splat(1.0) / t36 / t905;
            let t1637 = t914 * t1636;
            let t1639 = t50 * t28 * t32;
            let t1646 = f64x8::splat(3.519518943150165e-07) * t1637 * t1639 - f64x8::splat(0.00011403241375806533) * t428 * t1592 + f64x8::splat(0.0014778600823045268) * t185 * t632;
            let t1653 = -f64x8::splat(2.0) * t177 * t1605 + f64x8::splat(0.00018245186201290453) * t385 * t632 + f64x8::splat(4.0) * t1397 * t651 - f64x8::splat(6.0) * t875 * t1612 + f64x8::splat(4.0) * t413 * t1615 + f64x8::splat(4.0) * t413 * t1618 + f64x8::splat(2.0) * t413 * t1621 - t49 * t1624 * t76 / f64x8::splat(54.0) + t49 * t748 * t192 / f64x8::splat(288.0) - t763 * t1632 / f64x8::splat(288.0) + t69 * t1646 + f64x8::splat(2.0) * t645 * t309 + f64x8::splat(2.0) * t295 * t671 + t173 * t776;
            let t1654 = t1604 + t1653;
            let t1659 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t144 * t778 - t1570 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t1654));
            let tv3rhosigma20 = t6 * t1659 + t782;
            acc_v3rhosigma2_0 = tv3rhosigma20;
            let tv3rhosigma21 = f64x8::splat(0.0);
            acc_v3rhosigma2_1 = tv3rhosigma21;
            let tv3rhosigma22 = f64x8::splat(0.0);
            acc_v3rhosigma2_2 = tv3rhosigma22;
            let tv3rhosigma23 = f64x8::splat(0.0);
            acc_v3rhosigma2_3 = tv3rhosigma23;
            let tv3rhosigma24 = f64x8::splat(0.0);
            acc_v3rhosigma2_4 = tv3rhosigma24;
            let t1666 = t5 * t210 * t815 / f64x8::splat(8.0);
            let t1668 = ((t83).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t206 * t815 - t1666));
            let tv3rhosigma25 = t6 * t1668 + t819;
            acc_v3rhosigma2_5 = tv3rhosigma25;
            let t1674 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t223 * t778 - t1570));
            let tv3rhosigma26 = t6 * t1674 + t782;
            acc_v3rhosigma2_6 = tv3rhosigma26;
            let tv3rhosigma27 = f64x8::splat(0.0);
            acc_v3rhosigma2_7 = tv3rhosigma27;
            let tv3rhosigma28 = f64x8::splat(0.0);
            acc_v3rhosigma2_8 = tv3rhosigma28;
            let tv3rhosigma29 = f64x8::splat(0.0);
            acc_v3rhosigma2_9 = tv3rhosigma29;
            let tv3rhosigma210 = f64x8::splat(0.0);
            acc_v3rhosigma2_10 = tv3rhosigma210;
            let t1683 = t737 * t330;
            let t1686 = t339 * t724;
            let t1700 = t276 * t793;
            let t1703 = t276 * t805;
            let t1705 = t49 * t252;
            let t1709 = -t1705 / f64x8::splat(54.0) - t802 * t263 / f64x8::splat(48.0);
            let t1710 = t129 * t1709;
            let t1712 = f64x8::splat(0.00018245186201290453) * t565 * t698 - f64x8::splat(2.0) * t586 * t797 - f64x8::splat(2.0) * t261 * t1683 - f64x8::splat(2.0) * t261 * t1686 - t586 * t806 - f64x8::splat(2.0) * t1482 * t331 - f64x8::splat(2.0) * t713 * t720 - f64x8::splat(2.0) * t713 * t725 - f64x8::splat(2.0) * t713 * t728 + f64x8::splat(2.0) * t1205 * t794 + f64x8::splat(2.0) * t593 * t1700 - t261 * t1703 - t261 * t1710;
            let t1713 = t813 * t266;
            let t1715 = t263 * v_sigma2;
            let t1718 = t797 * t266;
            let t1721 = t806 * t266;
            let t1729 = t794 * t266;
            let t1732 = t331 * t724;
            let t1735 = t252 * t121;
            let t1739 = t260 * t129;
            let t1740 = t1739 * t266;
            let t1749 = f64x8::splat(1.0) / t95 / t1247;
            let t1750 = t1237 * t1749;
            let t1752 = t105 * t28 * t32;
            let t1759 = f64x8::splat(3.519518943150165e-07) * t1750 * t1752 - f64x8::splat(0.00011403241375806533) * t608 * t1715 + f64x8::splat(0.0014778600823045268) * t269 * t698;
            let t1761 = -t261 * t1713 - f64x8::splat(8.446845463560395e-06) * t1237 * t1715 + f64x8::splat(4.0) * t593 * t1718 + f64x8::splat(2.0) * t593 * t1721 + t49 * t785 * t276 / f64x8::splat(288.0) + f64x8::splat(4.0) * t1527 * t717 - f64x8::splat(6.0) * t1210 * t1729 + f64x8::splat(4.0) * t593 * t1732 - t49 * t1735 * t129 / f64x8::splat(54.0) - t800 * t1740 / f64x8::splat(288.0) + f64x8::splat(2.0) * t711 * t339 + f64x8::splat(2.0) * t325 * t737 + t257 * t813 + t122 * t1759;
            let t1762 = t1712 + t1761;
            let t1767 = ((t83).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t234 * t815 - t1666 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t92 * t1762));
            let tv3rhosigma211 = t6 * t1767 + t819;
            acc_v3rhosigma2_11 = tv3rhosigma211;
            let t1769 = t301 * t768;
            let t1772 = t76 * t60;
            let t1773 = t1772 * t64;
            let t1781 = t756 * t300;
            let t1782 = t76 * t1781;
            let t1785 = t309 * t768;
            let t1792 = t309 * t756;
            let t1795 = t776 * t300;
            let t1798 = t1631 * t300;
            let t1806 = f64x8::splat(1.0) / t36 / t421;
            let t1809 = v_sigma0 * t28 * t32;
            let t1814 = -f64x8::splat(1.3198196036813117e-07) * t914 * t1806 * t1809 + f64x8::splat(2.56572930955647e-05) * t428 * t64;
            let t1816 = f64x8::splat(6.0) * t413 * t1769 - t177 * t1773 / f64x8::splat(384.0) + t49 * t748 * t309 / f64x8::splat(96.0) + f64x8::splat(6.0) * t1397 * t757 - f64x8::splat(6.0) * t875 * t1782 - f64x8::splat(3.0) * t177 * t1785 - f64x8::splat(6.0) * t647 * t760 - f64x8::splat(3.0) * t647 * t769 + f64x8::splat(6.0) * t413 * t1792 - f64x8::splat(3.0) * t177 * t1795 - t763 * t1798 / f64x8::splat(96.0) + f64x8::splat(3.167567048835148e-06) * t914 * t64 + f64x8::splat(3.0) * t295 * t776 + t69 * t1814;
            let t1820 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t1816));
            let tv3sigma30 = t6 * t1820;
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
            let t1821 = t129 * t60;
            let t1822 = t1821 * t117;
            let t1830 = t793 * t330;
            let t1831 = t129 * t1830;
            let t1834 = t331 * t805;
            let t1837 = t339 * t805;
            let t1844 = t339 * t793;
            let t1847 = t813 * t330;
            let t1850 = t1739 * t330;
            let t1856 = f64x8::splat(1.0) / t95 / t601;
            let t1859 = v_sigma2 * t28 * t32;
            let t1864 = -f64x8::splat(1.3198196036813117e-07) * t1237 * t1856 * t1859 + f64x8::splat(2.56572930955647e-05) * t608 * t117;
            let t1868 = -t261 * t1822 / f64x8::splat(384.0) + t49 * t785 * t339 / f64x8::splat(96.0) + f64x8::splat(6.0) * t1527 * t794 - f64x8::splat(6.0) * t1210 * t1831 + f64x8::splat(6.0) * t593 * t1834 - f64x8::splat(3.0) * t261 * t1837 - f64x8::splat(6.0) * t713 * t797 - f64x8::splat(3.0) * t713 * t806 + f64x8::splat(6.0) * t593 * t1844 - f64x8::splat(3.0) * t261 * t1847 - t800 * t1850 / f64x8::splat(96.0) + f64x8::splat(3.0) * t325 * t813 + t122 * t1864 + f64x8::splat(3.167567048835148e-06) * t1237 * t117;
            let t1872 = ((t83).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t92 * t1868));
            let tv3sigma39 = t6 * t1872;
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
