//! GGA_K_VT84F lxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_vt84f.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_vt84f_lxc_unpol(
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
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4sigma4: &mut [f64],
    param_alpha: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_alpha = f64x8::splat(param_alpha);
    let param_mu = f64x8::splat(param_mu);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        let mut acc_v3rho3 = V_ZERO;
        let mut acc_v3rho2sigma = V_ZERO;
        let mut acc_v3rhosigma2 = V_ZERO;
        let mut acc_v3sigma3 = V_ZERO;
        let mut acc_v4rho4 = V_ZERO;
        let mut acc_v4rho3sigma = V_ZERO;
        let mut acc_v4rho2sigma2 = V_ZERO;
        let mut acc_v4rhosigma3 = V_ZERO;
        let mut acc_v4sigma4 = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = t3 * t3;
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 * t5 * f64x8::splat(M_PI);
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t15 = t14 * t14;
            let t17 = (simd::cbrt(t12));
            let t18 = t17 * t17;
            let t20 = (((t12).simd_le(zeta_threshold)).select(t15 * zeta_threshold, t18 * t12));
            let t21 = (simd::cbrt(v_rho));
            let t22 = t21 * t21;
            let t23 = t20 * t22;
            let t24 = f64x8::splat(M_CBRT6);
            let t25 = t24 * t24;
            let t26 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t27 = (simd::cbrt(t26));
            let t29 = t25 / t27;
            let t30 = ((v_sigma).sqrt());
            let t31 = f64x8::splat(M_CBRT2);
            let t32 = t30 * t31;
            let t34 = f64x8::splat(1.0) / t21 / v_rho;
            let t37 = t29 * t32 * t34 / f64x8::splat(12.0);
            let t38 = ((f64x8::splat(f64::EPSILON)).sqrt());
            let t39 = (t37).simd_le(t38);
            let t41 = (-param_mu + param_alpha + f64x8::splat(5.0) / f64x8::splat(3.0)) * t24;
            let t42 = t27 * t27;
            let t43 = f64x8::splat(1.0) / t42;
            let t44 = t41 * t43;
            let t45 = t31 * t31;
            let t46 = v_sigma * t45;
            let t47 = v_rho * v_rho;
            let t49 = f64x8::splat(1.0) / t22 / t47;
            let t53 = param_mu * param_alpha;
            let t54 = param_mu * param_mu;
            let t56 = (t53 + t54 - param_alpha) * t25;
            let t58 = f64x8::splat(1.0) / t27 / t26;
            let t59 = t56 * t58;
            let t60 = v_sigma * v_sigma;
            let t61 = t60 * t31;
            let t62 = t47 * t47;
            let t63 = t62 * v_rho;
            let t65 = f64x8::splat(1.0) / t21 / t63;
            let t69 = param_alpha * param_alpha;
            let t71 = param_mu * t69 / f64x8::splat(2.0);
            let t74 = t69 / f64x8::splat(2.0);
            let t76 = t26 * t26;
            let t78 = (-t71 - (t53 + t54) * param_mu - t74) / t76;
            let t79 = t60 * v_sigma;
            let t80 = t62 * t62;
            let t81 = f64x8::splat(1.0) / t80;
            let t85 = t69 * param_alpha;
            let t89 = t54 * param_mu;
            let t93 = (param_mu * t85 / f64x8::splat(6.0) - (-param_alpha * t54 - t71 - t89) * param_mu + t74) * t24;
            let t95 = f64x8::splat(1.0) / t42 / t76;
            let t96 = t93 * t95;
            let t97 = t60 * t60;
            let t98 = t97 * t45;
            let t99 = t80 * t47;
            let t101 = f64x8::splat(1.0) / t22 / t99;
            let t106 = (t38).simd_lt(t37);
            let t107 = ((t106).select(t37, t38));
            let t108 = t107 * t107;
            let t109 = param_mu * t108;
            let t110 = param_alpha * t108;
            let t111 = (simd::exp(-t110));
            let t112 = f64x8::splat(1.0) + t109;
            let t113 = f64x8::splat(1.0) / t112;
            let t114 = t111 * t113;
            let t116 = t108 * t108;
            let t118 = (simd::exp(-param_alpha * t116));
            let t119 = f64x8::splat(1.0) - t118;
            let t120 = f64x8::splat(1.0) / t108;
            let t121 = t120 - f64x8::splat(1.0);
            let t125 = ((t39).select(f64x8::splat(1.0) + t44 * t46 * t49 / f64x8::splat(24.0) + t59 * t61 * t65 / f64x8::splat(288.0) + t78 * t79 * t81 / f64x8::splat(576.0) + t96 * t98 * t101 / f64x8::splat(13824.0), f64x8::splat(1.0) - t109 * t114 + t119 * t121 + f64x8::splat(5.0) / f64x8::splat(3.0) * t108));
            let t129 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t125));
            let tzk0 = f64x8::splat(2.0) * t129;
            acc_zk = tzk0;
            let t131 = t20 / t21;
            let t135 = t47 * v_rho;
            let t137 = f64x8::splat(1.0) / t22 / t135;
            let t141 = t62 * t47;
            let t143 = f64x8::splat(1.0) / t21 / t141;
            let t147 = t80 * v_rho;
            let t148 = f64x8::splat(1.0) / t147;
            let t152 = t80 * t135;
            let t154 = f64x8::splat(1.0) / t22 / t152;
            let t159 = param_mu * t107;
            let t161 = f64x8::splat(1.0) / t21 / t47;
            let t165 = ((t106).select(-t29 * t32 * t161 / f64x8::splat(9.0), f64x8::splat(0.0)));
            let t166 = t114 * t165;
            let t169 = t108 * t107;
            let t170 = param_mu * t169;
            let t171 = t170 * param_alpha;
            let t174 = t54 * t169;
            let t175 = t112 * t112;
            let t176 = f64x8::splat(1.0) / t175;
            let t177 = t111 * t176;
            let t178 = t177 * t165;
            let t181 = param_alpha * t169;
            let t182 = t165 * t118;
            let t183 = t182 * t121;
            let t187 = t119 / t169;
            let t190 = t107 * t165;
            let t193 = ((t39).select(-t44 * t46 * t137 / f64x8::splat(9.0) - t59 * t61 * t143 / f64x8::splat(54.0) - t78 * t79 * t148 / f64x8::splat(72.0) - t96 * t98 * t154 / f64x8::splat(1296.0), -f64x8::splat(2.0) * t159 * t166 + f64x8::splat(2.0) * t171 * t166 + f64x8::splat(2.0) * t174 * t178 + f64x8::splat(4.0) * t181 * t183 - f64x8::splat(2.0) * t187 * t165 + f64x8::splat(10.0) / f64x8::splat(3.0) * t190));
            let t198 = ((t2).select(f64x8::splat(0.0), t7 * t131 * t125 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t193));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t198 + f64x8::splat(2.0) * t129;
            acc_vrho = tvrho0;
            let t201 = t43 * t45;
            let t205 = v_sigma * t31;
            let t212 = t79 * t45;
            let t218 = f64x8::splat(1.0) / t30 * t31;
            let t222 = ((t106).select(t29 * t218 * t34 / f64x8::splat(24.0), f64x8::splat(0.0)));
            let t223 = t114 * t222;
            let t228 = t177 * t222;
            let t231 = t222 * t118;
            let t232 = t231 * t121;
            let t240 = ((t39).select(t41 * t201 * t49 / f64x8::splat(24.0) + t59 * t205 * t65 / f64x8::splat(144.0) + t78 * t60 * t81 / f64x8::splat(192.0) + t96 * t212 * t101 / f64x8::splat(3456.0), -f64x8::splat(2.0) * t159 * t223 + f64x8::splat(2.0) * t171 * t223 + f64x8::splat(2.0) * t174 * t228 + f64x8::splat(4.0) * t181 * t232 - f64x8::splat(2.0) * t187 * t222 + f64x8::splat(10.0) / f64x8::splat(3.0) * t107 * t222));
            let t244 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t240));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t244;
            acc_vsigma = tvsigma0;
            let t247 = t20 * t34;
            let t255 = f64x8::splat(1.0) / t22 / t62;
            let t261 = f64x8::splat(1.0) / t21 / t62 / t135;
            let t265 = f64x8::splat(1.0) / t99;
            let t269 = t80 * t62;
            let t271 = f64x8::splat(1.0) / t22 / t269;
            let t276 = t165 * t165;
            let t277 = param_mu * t276;
            let t280 = t109 * param_alpha;
            let t281 = t276 * t111;
            let t282 = t281 * t113;
            let t285 = t54 * t108;
            let t286 = t177 * t276;
            let t290 = f64x8::splat(1.0) / t21 / t135;
            let t294 = ((t106).select(f64x8::splat(7.0) / f64x8::splat(27.0) * t29 * t32 * t290, f64x8::splat(0.0)));
            let t295 = t114 * t294;
            let t300 = param_mu * t116;
            let t301 = t300 * t69;
            let t304 = t54 * t116;
            let t305 = t304 * param_alpha;
            let t308 = t89 * t116;
            let t310 = f64x8::splat(1.0) / t175 / t112;
            let t311 = t111 * t310;
            let t319 = t276 * t118 * t121;
            let t322 = t294 * t118;
            let t323 = t322 * t121;
            let t326 = t116 * t108;
            let t327 = t69 * t326;
            let t330 = param_alpha * t276;
            let t334 = t119 / t116;
            let t340 = t107 * t294;
            let t342 = -f64x8::splat(2.0) * t277 * t114 + f64x8::splat(10.0) * t280 * t282 + f64x8::splat(10.0) * t285 * t286 - f64x8::splat(2.0) * t159 * t295 + f64x8::splat(2.0) * t171 * t295 - f64x8::splat(4.0) * t301 * t282 - f64x8::splat(8.0) * t305 * t286 - f64x8::splat(8.0) * t308 * t311 * t276 + f64x8::splat(2.0) * t174 * t177 * t294 + f64x8::splat(12.0) * t110 * t319 + f64x8::splat(4.0) * t181 * t323 - f64x8::splat(16.0) * t327 * t319 - f64x8::splat(16.0) * t330 * t118 + f64x8::splat(6.0) * t334 * t276 - f64x8::splat(2.0) * t187 * t294 + f64x8::splat(10.0) / f64x8::splat(3.0) * t276 + f64x8::splat(10.0) / f64x8::splat(3.0) * t340;
            let t343 = ((t39).select(f64x8::splat(11.0) / f64x8::splat(27.0) * t44 * t46 * t255 + f64x8::splat(19.0) / f64x8::splat(162.0) * t59 * t61 * t261 + t78 * t79 * t265 / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(3888.0) * t96 * t98 * t271, t342));
            let t348 = ((t2).select(f64x8::splat(0.0), -t7 * t247 * t125 / f64x8::splat(30.0) + t7 * t131 * t193 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t343));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t348 + f64x8::splat(4.0) * t198;
            acc_v2rho2 = tv2rho20;
            let t367 = param_mu * t165;
            let t370 = t165 * t111;
            let t371 = t113 * t222;
            let t372 = t370 * t371;
            let t375 = t285 * t111;
            let t376 = t176 * t222;
            let t383 = ((t106).select(-t29 * t218 * t161 / f64x8::splat(18.0), f64x8::splat(0.0)));
            let t384 = t114 * t383;
            let t391 = t222 * t111;
            let t392 = t176 * t165;
            let t393 = t391 * t392;
            let t396 = t308 * t111;
            let t397 = t310 * t222;
            let t404 = t110 * t222;
            let t407 = t383 * t118;
            let t408 = t407 * t121;
            let t411 = t327 * t222;
            let t414 = param_alpha * t222;
            let t417 = t165 * t222;
            let t425 = -f64x8::splat(2.0) * t367 * t223 + f64x8::splat(10.0) * t280 * t372 + f64x8::splat(10.0) * t375 * t376 * t165 - f64x8::splat(2.0) * t159 * t384 + f64x8::splat(2.0) * t171 * t384 - f64x8::splat(4.0) * t301 * t372 - f64x8::splat(8.0) * t305 * t393 - f64x8::splat(8.0) * t396 * t397 * t165 + f64x8::splat(2.0) * t174 * t177 * t383 + f64x8::splat(12.0) * t404 * t183 + f64x8::splat(4.0) * t181 * t408 - f64x8::splat(16.0) * t411 * t183 - f64x8::splat(16.0) * t414 * t182 + f64x8::splat(6.0) * t334 * t417 - f64x8::splat(2.0) * t187 * t383 + f64x8::splat(10.0) / f64x8::splat(3.0) * t417 + f64x8::splat(10.0) / f64x8::splat(3.0) * t107 * t383;
            let t426 = ((t39).select(-t41 * t201 * t137 / f64x8::splat(9.0) - t59 * t205 * t143 / f64x8::splat(27.0) - t78 * t60 * t148 / f64x8::splat(24.0) - t96 * t212 * t154 / f64x8::splat(324.0), t425));
            let t431 = ((t2).select(f64x8::splat(0.0), t7 * t131 * t240 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t426));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t431 + f64x8::splat(2.0) * t244;
            acc_v2rhosigma = tv2rhosigma0;
            let t434 = t58 * t31;
            let t441 = t60 * t45;
            let t446 = t222 * t222;
            let t447 = param_mu * t446;
            let t450 = t446 * t111;
            let t451 = t450 * t113;
            let t454 = t177 * t446;
            let t459 = f64x8::splat(1.0) / t30 / v_sigma * t31;
            let t463 = ((t106).select(-t29 * t459 * t34 / f64x8::splat(48.0), f64x8::splat(0.0)));
            let t464 = t114 * t463;
            let t479 = t446 * t118;
            let t480 = t479 * t121;
            let t484 = t463 * t118 * t121;
            let t499 = -f64x8::splat(2.0) * t447 * t114 + f64x8::splat(10.0) * t280 * t451 + f64x8::splat(10.0) * t285 * t454 - f64x8::splat(2.0) * t159 * t464 + f64x8::splat(2.0) * t171 * t464 - f64x8::splat(4.0) * t301 * t451 - f64x8::splat(8.0) * t305 * t454 - f64x8::splat(8.0) * t308 * t311 * t446 + f64x8::splat(2.0) * t174 * t177 * t463 + f64x8::splat(12.0) * t110 * t480 + f64x8::splat(4.0) * t181 * t484 - f64x8::splat(16.0) * t327 * t480 - f64x8::splat(16.0) * param_alpha * t446 * t118 + f64x8::splat(6.0) * t334 * t446 - f64x8::splat(2.0) * t187 * t463 + f64x8::splat(10.0) / f64x8::splat(3.0) * t446 + f64x8::splat(10.0) / f64x8::splat(3.0) * t107 * t463;
            let t500 = ((t39).select(t56 * t434 * t65 / f64x8::splat(144.0) + t78 * v_sigma * t81 / f64x8::splat(96.0) + t96 * t441 * t101 / f64x8::splat(1152.0), t499));
            let t504 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t500));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t504;
            acc_v2sigma2 = tv2sigma20;
            let t507 = t20 * t161;
            let t518 = f64x8::splat(1.0) / t22 / t63;
            let t523 = f64x8::splat(1.0) / t21 / t80;
            let t527 = f64x8::splat(1.0) / t152;
            let t533 = f64x8::splat(1.0) / t22 / t80 / t63;
            let t538 = t165 * t294;
            let t541 = f64x8::splat(1.0) / t21 / t62;
            let t545 = ((t106).select(-f64x8::splat(70.0) / f64x8::splat(81.0) * t29 * t32 * t541, f64x8::splat(0.0)));
            let t548 = t114 * t545;
            let t551 = t54 * t54;
            let t552 = t116 * t107;
            let t553 = t551 * t552;
            let t554 = t175 * t175;
            let t555 = f64x8::splat(1.0) / t554;
            let t556 = t111 * t555;
            let t557 = t276 * t165;
            let t564 = t69 * t552;
            let t565 = t557 * t118;
            let t566 = t565 * t121;
            let t569 = t545 * t118;
            let t570 = t569 * t121;
            let t573 = t116 * t116;
            let t575 = t85 * t573 * t107;
            let t580 = t54 * t557;
            let t581 = t177 * t107;
            let t584 = t89 * t169;
            let t585 = t311 * t557;
            let t588 = param_alpha * t107;
            let t592 = t119 / t552;
            let t597 = param_mu * t557;
            let t598 = t597 * param_alpha;
            let t599 = t107 * t111;
            let t600 = t599 * t113;
            let t603 = f64x8::splat(10.0) * t538 + f64x8::splat(10.0) / f64x8::splat(3.0) * t107 * t545 - f64x8::splat(2.0) * t159 * t548 + f64x8::splat(48.0) * t553 * t556 * t557 + f64x8::splat(2.0) * t174 * t177 * t545 - f64x8::splat(144.0) * t564 * t566 + f64x8::splat(4.0) * t181 * t570 + f64x8::splat(64.0) * t575 * t566 - f64x8::splat(6.0) * t367 * t295 + f64x8::splat(24.0) * t580 * t581 - f64x8::splat(72.0) * t584 * t585 + f64x8::splat(24.0) * t588 * t566 - f64x8::splat(24.0) * t592 * t557 - f64x8::splat(2.0) * t187 * t545 + f64x8::splat(24.0) * t598 * t600;
            let t604 = t170 * t69;
            let t605 = t557 * t111;
            let t606 = t605 * t113;
            let t609 = t174 * param_alpha;
            let t610 = t605 * t176;
            let t618 = param_mu * t552;
            let t619 = t618 * t85;
            let t622 = t54 * t552;
            let t623 = t622 * t69;
            let t626 = t89 * t552;
            let t627 = t626 * param_alpha;
            let t630 = t310 * t165;
            let t640 = t113 * t294;
            let t641 = t370 * t640;
            let t646 = t294 * t111;
            let t647 = t646 * t392;
            let t653 = t69 * t169;
            let t658 = f64x8::splat(36.0) * t110 * t165 * t323 - f64x8::splat(48.0) * param_alpha * t294 * t182 - f64x8::splat(48.0) * t327 * t294 * t183 + f64x8::splat(30.0) * t375 * t392 * t294 - f64x8::splat(24.0) * t396 * t630 * t294 + f64x8::splat(2.0) * t171 * t548 + f64x8::splat(30.0) * t280 * t641 - f64x8::splat(12.0) * t301 * t641 - f64x8::splat(24.0) * t305 * t647 + f64x8::splat(18.0) * t334 * t538 + f64x8::splat(96.0) * t653 * t565 + f64x8::splat(48.0) * t627 * t585 - f64x8::splat(36.0) * t604 * t606 + f64x8::splat(8.0) * t619 * t606 - f64x8::splat(72.0) * t609 * t610 + f64x8::splat(24.0) * t623 * t610;
            let t660 = ((t39).select(-f64x8::splat(154.0) / f64x8::splat(81.0) * t44 * t46 * t518 - f64x8::splat(209.0) / f64x8::splat(243.0) * t59 * t61 * t523 - f64x8::splat(5.0) / f64x8::splat(4.0) * t78 * t79 * t527 - f64x8::splat(665.0) / f64x8::splat(5832.0) * t96 * t98 * t533, t603 + t658));
            let t665 = ((t2).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(45.0) * t7 * t507 * t125 - t7 * t247 * t193 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(10.0) * t7 * t131 * t343 + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t660));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t665 + f64x8::splat(6.0) * t348;
            acc_v3rho3 = tv3rho30;
            let t688 = t165 * t383;
            let t693 = ((t106).select(f64x8::splat(7.0) / f64x8::splat(54.0) * t29 * t218 * t290, f64x8::splat(0.0)));
            let t696 = t294 * t222;
            let t698 = param_mu * t294;
            let t703 = t114 * t693;
            let t709 = t693 * t118;
            let t710 = t709 * t121;
            let t713 = t222 * t276;
            let t720 = t588 * t222;
            let t723 = t310 * t383;
            let t730 = t553 * t111;
            let t731 = t555 * t222;
            let t735 = t110 * t383;
            let t740 = t564 * t222;
            let t743 = t327 * t383;
            let t748 = t575 * t222;
            let t752 = t54 * t276 * t111;
            let t756 = f64x8::splat(24.0) * t752 * t376 * t107 - f64x8::splat(16.0) * t396 * t723 * t165 + f64x8::splat(48.0) * t730 * t731 * t276 - f64x8::splat(8.0) * t396 * t397 * t294 + f64x8::splat(24.0) * t735 * t183 - f64x8::splat(32.0) * t743 * t183 + f64x8::splat(24.0) * t720 * t319 - f64x8::splat(144.0) * t740 * t319 + f64x8::splat(64.0) * t748 * t319 + f64x8::splat(12.0) * t404 * t323 - f64x8::splat(16.0) * t411 * t323;
            let t758 = t176 * t383;
            let t765 = t584 * t111;
            let t771 = t688 * t114;
            let t774 = t383 * t111;
            let t775 = t774 * t392;
            let t778 = t696 * t114;
            let t781 = t713 * t114;
            let t784 = t713 * t177;
            let t787 = t176 * t294;
            let t788 = t391 * t787;
            let t792 = t310 * t276;
            let t796 = t277 * param_alpha;
            let t810 = param_alpha * t383;
            let t819 = f64x8::splat(24.0) * t796 * t599 * t371 + f64x8::splat(48.0) * t627 * t391 * t792 - f64x8::splat(32.0) * t810 * t182 + f64x8::splat(20.0) * t280 * t771 + f64x8::splat(10.0) * t280 * t778 - f64x8::splat(16.0) * t414 * t322 + f64x8::splat(12.0) * t334 * t688 + f64x8::splat(6.0) * t334 * t696 - f64x8::splat(24.0) * t592 * t713 - f64x8::splat(36.0) * t604 * t781 - f64x8::splat(72.0) * t609 * t784;
            let t822 = ((t39).select(f64x8::splat(11.0) / f64x8::splat(27.0) * t41 * t201 * t255 + f64x8::splat(19.0) / f64x8::splat(81.0) * t59 * t205 * t261 + f64x8::splat(3.0) / f64x8::splat(8.0) * t78 * t60 * t265 + f64x8::splat(35.0) / f64x8::splat(972.0) * t96 * t212 * t271, f64x8::splat(20.0) / f64x8::splat(3.0) * t688 + f64x8::splat(10.0) / f64x8::splat(3.0) * t107 * t693 + f64x8::splat(10.0) / f64x8::splat(3.0) * t696 - f64x8::splat(2.0) * t698 * t223 - f64x8::splat(4.0) * t367 * t384 - f64x8::splat(2.0) * t159 * t703 + f64x8::splat(2.0) * t174 * t177 * t693 + f64x8::splat(4.0) * t181 * t710 + f64x8::splat(96.0) * t653 * t713 * t118 - f64x8::splat(2.0) * t187 * t693 + t756 + f64x8::splat(20.0) * t375 * t758 * t165 - f64x8::splat(72.0) * t765 * t397 * t276 + f64x8::splat(10.0) * t375 * t376 * t294 + f64x8::splat(2.0) * t171 * t703 - f64x8::splat(8.0) * t301 * t771 - f64x8::splat(4.0) * t301 * t778 - f64x8::splat(16.0) * t305 * t775 - f64x8::splat(8.0) * t305 * t788 + f64x8::splat(8.0) * t619 * t781 + f64x8::splat(24.0) * t623 * t784 + t819));
            let t827 = ((t2).select(f64x8::splat(0.0), -t7 * t247 * t240 / f64x8::splat(30.0) + t7 * t131 * t426 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t822));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t827 + f64x8::splat(4.0) * t431;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t843 = t446 * t165;
            let t844 = t843 * t114;
            let t847 = t450 * t392;
            let t850 = t113 * t463;
            let t851 = t370 * t850;
            let t856 = t463 * t111;
            let t857 = t856 * t392;
            let t860 = t113 * t383;
            let t861 = t391 * t860;
            let t868 = t391 * t758;
            let t875 = t447 * param_alpha;
            let t881 = t222 * t383;
            let t883 = t165 * t463;
            let t888 = ((t106).select(t29 * t459 * t161 / f64x8::splat(36.0), f64x8::splat(0.0)));
            let t895 = t888 * t118 * t121;
            let t901 = param_mu * t222;
            let t906 = t114 * t888;
            let t909 = f64x8::splat(24.0) * t875 * t190 * t114 + f64x8::splat(20.0) * t280 * t861 + f64x8::splat(20.0) / f64x8::splat(3.0) * t881 + f64x8::splat(10.0) / f64x8::splat(3.0) * t883 + f64x8::splat(10.0) / f64x8::splat(3.0) * t107 * t888 + f64x8::splat(2.0) * t174 * t177 * t888 + f64x8::splat(4.0) * t181 * t895 + f64x8::splat(96.0) * t653 * t479 * t165 - f64x8::splat(4.0) * t901 * t384 - f64x8::splat(2.0) * t367 * t464 - f64x8::splat(2.0) * t159 * t906;
            let t913 = t564 * t446;
            let t916 = t110 * t463;
            let t919 = t327 * t463;
            let t924 = t575 * t446;
            let t927 = t54 * t446;
            let t928 = t927 * t111;
            let t929 = t176 * t107;
            let t936 = t310 * t446;
            let t940 = t176 * t463;
            let t950 = t555 * t446;
            let t954 = t310 * t463;
            let t958 = t588 * t446;
            let t963 = param_alpha * t463;
            let t974 = -f64x8::splat(8.0) * t396 * t954 * t165 + f64x8::splat(48.0) * t730 * t950 * t165 - f64x8::splat(16.0) * t396 * t397 * t383 + f64x8::splat(2.0) * t171 * t906 - f64x8::splat(16.0) * t963 * t182 + f64x8::splat(24.0) * t958 * t183 + f64x8::splat(12.0) * t334 * t881 + f64x8::splat(6.0) * t334 * t883 + f64x8::splat(24.0) * t404 * t408 - f64x8::splat(32.0) * t414 * t407 - f64x8::splat(24.0) * t592 * t843;
            let t977 = ((t39).select(-t56 * t434 * t143 / f64x8::splat(27.0) - t78 * v_sigma * t148 / f64x8::splat(12.0) - t96 * t441 * t154 / f64x8::splat(108.0), f64x8::splat(48.0) * t627 * t450 * t630 + f64x8::splat(10.0) * t280 * t851 - f64x8::splat(4.0) * t301 * t851 - f64x8::splat(8.0) * t301 * t861 - f64x8::splat(8.0) * t305 * t857 - f64x8::splat(16.0) * t305 * t868 - f64x8::splat(36.0) * t604 * t844 - f64x8::splat(72.0) * t609 * t847 + f64x8::splat(8.0) * t619 * t844 + f64x8::splat(24.0) * t623 * t847 + t909 + f64x8::splat(10.0) * t375 * t940 * t165 - f64x8::splat(72.0) * t765 * t936 * t165 + f64x8::splat(24.0) * t928 * t929 * t165 + f64x8::splat(20.0) * t375 * t376 * t383 - f64x8::splat(144.0) * t913 * t183 + f64x8::splat(12.0) * t916 * t183 - f64x8::splat(16.0) * t919 * t183 + f64x8::splat(64.0) * t924 * t183 - f64x8::splat(2.0) * t187 * t888 - f64x8::splat(32.0) * t411 * t408 + t974));
            let t982 = ((t2).select(f64x8::splat(0.0), t7 * t131 * t500 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t977));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t982 + f64x8::splat(2.0) * t504;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t991 = t391 * t850;
            let t999 = t222 * t463;
            let t1003 = f64x8::splat(1.0) / t30 / t60 * t31;
            let t1007 = ((t106).select(t29 * t1003 * t34 / f64x8::splat(32.0), f64x8::splat(0.0)));
            let t1010 = t446 * t222;
            let t1011 = t1010 * t118;
            let t1012 = t1011 * t121;
            let t1017 = t54 * t1010;
            let t1020 = t311 * t1010;
            let t1023 = t114 * t1007;
            let t1035 = t1007 * t118 * t121;
            let t1040 = f64x8::splat(30.0) * t280 * t991 - f64x8::splat(12.0) * t301 * t991 - f64x8::splat(24.0) * t305 * t856 * t376 + f64x8::splat(10.0) * t999 + f64x8::splat(10.0) / f64x8::splat(3.0) * t107 * t1007 + f64x8::splat(24.0) * t588 * t1012 - f64x8::splat(6.0) * t901 * t464 + f64x8::splat(24.0) * t1017 * t581 - f64x8::splat(72.0) * t584 * t1020 - f64x8::splat(2.0) * t159 * t1023 + f64x8::splat(48.0) * t553 * t556 * t1010 + f64x8::splat(2.0) * t174 * t177 * t1007 - f64x8::splat(144.0) * t564 * t1012 + f64x8::splat(4.0) * t181 * t1035 + f64x8::splat(64.0) * t575 * t1012;
            let t1049 = t1010 * t111;
            let t1050 = t1049 * t113;
            let t1053 = t1049 * t176;
            let t1071 = param_mu * t1010 * param_alpha;
            let t1080 = f64x8::splat(30.0) * t375 * t376 * t463 - f64x8::splat(24.0) * t396 * t397 * t463 - f64x8::splat(2.0) * t187 * t1007 - f64x8::splat(24.0) * t592 * t1010 + f64x8::splat(96.0) * t653 * t1011 + f64x8::splat(48.0) * t627 * t1020 + f64x8::splat(2.0) * t171 * t1023 - f64x8::splat(36.0) * t604 * t1050 + f64x8::splat(8.0) * t619 * t1050 - f64x8::splat(72.0) * t609 * t1053 + f64x8::splat(24.0) * t623 * t1053 + f64x8::splat(24.0) * t1071 * t600 - f64x8::splat(48.0) * t963 * t231 - f64x8::splat(48.0) * t919 * t232 + f64x8::splat(18.0) * t334 * t999 + f64x8::splat(36.0) * t404 * t484;
            let t1082 = ((t39).select(t78 * t81 / f64x8::splat(96.0) + t96 * t46 * t101 / f64x8::splat(576.0), t1040 + t1080));
            let t1086 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t1082));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t1086;
            acc_v3sigma3 = tv3sigma30;
            let t1123 = t294 * t294;
            let t1125 = t165 * t545;
            let t1126 = t1125 * t114;
            let t1129 = t294 * t276;
            let t1130 = t1129 * t114;
            let t1133 = t1129 * t177;
            let t1141 = t176 * t545;
            let t1145 = t599 * t640;
            let t1156 = ((t106).select(f64x8::splat(910.0) / f64x8::splat(243.0) * t29 * t32 * t65, f64x8::splat(0.0)));
            let t1163 = t1123 * t118 * t121;
            let t1168 = t276 * t276;
            let t1169 = param_mu * t1168;
            let t1171 = param_alpha * t111 * t113;
            let t1174 = f64x8::splat(10.0) * t1123 - f64x8::splat(16.0) * t301 * t1126 + f64x8::splat(48.0) * t619 * t1130 + f64x8::splat(144.0) * t623 * t1133 + f64x8::splat(288.0) * t627 * t646 * t792 + f64x8::splat(40.0) * t280 * t1126 - f64x8::splat(32.0) * t305 * t370 * t1141 + f64x8::splat(144.0) * t796 * t1145 - f64x8::splat(216.0) * t604 * t1130 - f64x8::splat(432.0) * t609 * t1133 + f64x8::splat(40.0) / f64x8::splat(3.0) * t1125 + f64x8::splat(10.0) / f64x8::splat(3.0) * t107 * t1156 - f64x8::splat(24.0) * t308 * t311 * t1123 + f64x8::splat(36.0) * t110 * t1163 - f64x8::splat(48.0) * t327 * t1163 + f64x8::splat(24.0) * t1169 * t1171;
            let t1175 = t177 * t1123;
            let t1178 = t551 * t116;
            let t1179 = t556 * t1168;
            let t1182 = t69 * t116;
            let t1183 = t1168 * t118;
            let t1184 = t1183 * t121;
            let t1189 = t114 * t1156;
            let t1193 = t551 * param_mu * t326;
            let t1195 = f64x8::splat(1.0) / t554 / t112;
            let t1196 = t111 * t1195;
            let t1203 = t85 * t573;
            let t1210 = t69 * t69;
            let t1212 = t1210 * t573 * t116;
            let t1216 = t311 * t108;
            let t1221 = t169 * t276 * t118;
            let t1228 = t119 / t326;
            let t1235 = f64x8::splat(30.0) * t285 * t1175 + f64x8::splat(672.0) * t1178 * t1179 - f64x8::splat(816.0) * t1182 * t1184 - f64x8::splat(8.0) * t367 * t548 - f64x8::splat(2.0) * t159 * t1189 - f64x8::splat(384.0) * t1193 * t1196 * t1168 + f64x8::splat(2.0) * t174 * t177 * t1156 + f64x8::splat(1152.0) * t1203 * t1184 + f64x8::splat(4.0) * t181 * t1156 * t118 * t121 - f64x8::splat(256.0) * t1212 * t1184 - f64x8::splat(312.0) * t89 * t1168 * t1216 + f64x8::splat(576.0) * t69 * t294 * t1221 - f64x8::splat(48.0) * param_alpha * t1123 * t118 + f64x8::splat(120.0) * t1228 * t1168 + f64x8::splat(18.0) * t334 * t1123 - f64x8::splat(2.0) * t187 * t1156;
            let t1240 = t787 * t107;
            let t1243 = t54 * t1168;
            let t1245 = t108 * t111;
            let t1252 = t308 * param_alpha;
            let t1253 = t1168 * t111;
            let t1254 = t1253 * t310;
            let t1261 = t300 * t85;
            let t1262 = t1253 * t113;
            let t1265 = t304 * t69;
            let t1266 = t1253 * t176;
            let t1277 = t551 * t326 * param_alpha;
            let t1294 = t1123 * t111 * t113;
            let t1297 = f64x8::splat(384.0) * t575 * t276 * t323 + f64x8::splat(144.0) * t752 * t1240 - f64x8::splat(312.0) * t1243 * param_alpha * t1245 * t176 - f64x8::splat(432.0) * t765 * t792 * t294 + f64x8::splat(672.0) * t1252 * t1254 - f64x8::splat(156.0) * t1169 * t69 * t1245 * t113 + f64x8::splat(112.0) * t1261 * t1262 + f64x8::splat(336.0) * t1265 * t1266 + f64x8::splat(40.0) * t375 * t1141 * t165 + f64x8::splat(288.0) * t730 * t555 * t276 * t294 - f64x8::splat(384.0) * t1277 * t1179 - f64x8::splat(32.0) * t396 * t310 * t545 * t165 - f64x8::splat(864.0) * t564 * t276 * t323 + f64x8::splat(48.0) * t110 * t545 * t183 - f64x8::splat(64.0) * t327 * t545 * t183 + f64x8::splat(30.0) * t280 * t1294;
            let t1305 = param_mu * t326 * t1210;
            let t1309 = t54 * t326 * t85;
            let t1313 = t89 * t326 * t69;
            let t1320 = t118 * t121;
            let t1328 = t69 * t108;
            let t1334 = t85 * t326;
            let t1339 = param_alpha * t120;
            let t1344 = -f64x8::splat(12.0) * t301 * t1294 - f64x8::splat(24.0) * t305 * t1175 + f64x8::splat(2.0) * t171 * t1189 - f64x8::splat(16.0) * t1305 * t1262 - f64x8::splat(64.0) * t1309 * t1266 - f64x8::splat(192.0) * t1313 * t1254 + f64x8::splat(144.0) * t588 * t276 * t323 + f64x8::splat(24.0) * param_alpha * t1168 * t1320 - f64x8::splat(6.0) * param_mu * t1123 * t114 + f64x8::splat(24.0) * t1243 * t177 + f64x8::splat(576.0) * t1328 * t1183 - f64x8::splat(64.0) * param_alpha * t545 * t182 - f64x8::splat(512.0) * t1334 * t1183 - f64x8::splat(144.0) * t592 * t1129 - f64x8::splat(144.0) * t1339 * t1183 + f64x8::splat(24.0) * t334 * t1125;
            let t1347 = ((t39).select(f64x8::splat(2618.0) / f64x8::splat(243.0) * t44 * t46 / t22 / t141 + f64x8::splat(5225.0) / f64x8::splat(729.0) * t59 * t61 / t21 / t147 + f64x8::splat(55.0) / f64x8::splat(4.0) * t78 * t79 / t269 + f64x8::splat(27265.0) / f64x8::splat(17496.0) * t96 * t98 / t22 / t80 / t141, t1174 + t1235 + t1297 + t1344));
            let t1352 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(14.0) / f64x8::splat(135.0) * t7 * t20 * t290 * t125 + f64x8::splat(8.0) / f64x8::splat(45.0) * t7 * t507 * t193 - t7 * t247 * t343 / f64x8::splat(5.0) + f64x8::splat(2.0) / f64x8::splat(5.0) * t7 * t131 * t660 + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t1347));
            let tv4rho40 = f64x8::splat(2.0) * v_rho * t1352 + f64x8::splat(8.0) * t665;
            acc_v4rho4 = tv4rho40;
            let t1378 = t294 * t383;
            let t1381 = t545 * t222;
            let t1385 = t165 * t693;
            let t1390 = ((t106).select(-f64x8::splat(35.0) / f64x8::splat(81.0) * t29 * t218 * t541, f64x8::splat(0.0)));
            let t1394 = param_alpha * t165;
            let t1395 = t626 * t1394;
            let t1399 = t174 * t1394;
            let t1405 = t69 * t222;
            let t1406 = t170 * t1405;
            let t1410 = t618 * t85 * t222;
            let t1413 = f64x8::splat(18.0) * t334 * t1378 + f64x8::splat(6.0) * t334 * t1381 + f64x8::splat(10.0) * t1378 + f64x8::splat(10.0) * t1385 + f64x8::splat(10.0) / f64x8::splat(3.0) * t107 * t1390 + f64x8::splat(10.0) / f64x8::splat(3.0) * t1381 + f64x8::splat(144.0) * t1395 * t311 * t696 - f64x8::splat(216.0) * t1399 * t788 + f64x8::splat(72.0) * t698 * t588 * t372 - f64x8::splat(108.0) * t1406 * t641 + f64x8::splat(24.0) * t1410 * t641;
            let t1414 = t622 * t1405;
            let t1422 = t545 * t111 * t371;
            let t1425 = t646 * t860;
            let t1430 = t383 * t276;
            let t1431 = t1430 * t114;
            let t1434 = t1430 * t177;
            let t1442 = t222 * t557;
            let t1443 = t1442 * t114;
            let t1446 = t1442 * t177;
            let t1449 = -f64x8::splat(156.0) * t597 * t69 * t1245 * t371 - f64x8::splat(24.0) * t305 * t774 * t787 - f64x8::splat(16.0) * t1305 * t1443 - f64x8::splat(64.0) * t1309 * t1446 + f64x8::splat(72.0) * t1414 * t647 + f64x8::splat(10.0) * t280 * t1422 - f64x8::splat(4.0) * t301 * t1422 + f64x8::splat(30.0) * t280 * t1425 - f64x8::splat(12.0) * t301 * t1425 + f64x8::splat(24.0) * t619 * t1431 + f64x8::splat(72.0) * t623 * t1434;
            let t1451 = t1442 * t311;
            let t1465 = t693 * t111;
            let t1466 = t113 * t165;
            let t1467 = t1465 * t1466;
            let t1475 = t599 * t860;
            let t1481 = t1320 * t538;
            let t1484 = -f64x8::splat(312.0) * t580 * param_alpha * t1245 * t376 - f64x8::splat(8.0) * t305 * t391 * t1141 - f64x8::splat(384.0) * t1277 * t605 * t731 - f64x8::splat(24.0) * t305 * t1465 * t392 + f64x8::splat(672.0) * t1252 * t1451 - f64x8::splat(192.0) * t1313 * t1451 - f64x8::splat(216.0) * t609 * t1434 + f64x8::splat(30.0) * t280 * t1467 - f64x8::splat(12.0) * t301 * t1467 + f64x8::splat(72.0) * t796 * t1475 + f64x8::splat(72.0) * t720 * t1481;
            let t1511 = t1442 * t118;
            let t1514 = f64x8::splat(72.0) * t54 * t165 * t111 * t376 * t340 + f64x8::splat(144.0) * t627 * t281 * t723 - f64x8::splat(216.0) * t765 * t397 * t538 + f64x8::splat(144.0) * t730 * t731 * t538 + f64x8::splat(112.0) * t1261 * t1443 + f64x8::splat(336.0) * t1265 * t1446 + f64x8::splat(576.0) * t1328 * t1511 - f64x8::splat(108.0) * t604 * t1431 - f64x8::splat(432.0) * t740 * t1481 + f64x8::splat(192.0) * t748 * t1481 + f64x8::splat(24.0) * t580 * t228;
            let t1529 = t114 * t1390;
            let t1547 = f64x8::splat(4.0) * t181 * t1390 * t118 * t121 + f64x8::splat(288.0) * t653 * t1430 * t118 + f64x8::splat(2.0) * t174 * t177 * t1390 - f64x8::splat(2.0) * param_mu * t545 * t223 + f64x8::splat(24.0) * param_alpha * t557 * t232 - f64x8::splat(72.0) * t592 * t417 * t294 - f64x8::splat(512.0) * t1334 * t1511 - f64x8::splat(144.0) * t1339 * t1511 - f64x8::splat(2.0) * t159 * t1529 - f64x8::splat(6.0) * t367 * t703 - f64x8::splat(6.0) * t698 * t384;
            let t1552 = t1178 * t111;
            let t1570 = t176 * t693;
            let t1582 = -f64x8::splat(312.0) * t89 * t557 * t111 * t397 * t108 - f64x8::splat(816.0) * t1182 * t222 * t566 + f64x8::splat(672.0) * t1552 * t731 * t557 + f64x8::splat(30.0) * t375 * t1570 * t165 - f64x8::splat(216.0) * t765 * t723 * t276 + f64x8::splat(30.0) * t375 * t758 * t294 + f64x8::splat(72.0) * t588 * t383 * t319 + f64x8::splat(10.0) * t375 * t376 * t545 - f64x8::splat(2.0) * t187 * t1390 + f64x8::splat(2.0) * t171 * t1529 + f64x8::splat(24.0) * t598 * t223;
            let t1607 = t653 * t222;
            let t1614 = f64x8::splat(36.0) * t110 * t693 * t183 + f64x8::splat(1152.0) * t1203 * t222 * t566 - f64x8::splat(256.0) * t1212 * t222 * t566 + f64x8::splat(288.0) * t1607 * t322 * t165 - f64x8::splat(48.0) * t327 * t693 * t183 - f64x8::splat(432.0) * t564 * t383 * t319 + f64x8::splat(192.0) * t575 * t383 * t319 + f64x8::splat(36.0) * t735 * t323 - f64x8::splat(48.0) * t743 * t323 + f64x8::splat(12.0) * t404 * t570 - f64x8::splat(16.0) * t411 * t570;
            let t1615 = t758 * t107;
            let t1632 = t1193 * t111;
            let t1650 = -f64x8::splat(384.0) * t1632 * t1195 * t222 * t557 - f64x8::splat(24.0) * t396 * t310 * t693 * t165 + f64x8::splat(144.0) * t730 * t555 * t383 * t276 - f64x8::splat(48.0) * param_alpha * t693 * t182 - f64x8::splat(24.0) * t396 * t723 * t294 - f64x8::splat(8.0) * t396 * t397 * t545 + f64x8::splat(120.0) * t1228 * t1442 + f64x8::splat(18.0) * t334 * t1385 - f64x8::splat(72.0) * t592 * t1430 + f64x8::splat(72.0) * t752 * t1615 - f64x8::splat(48.0) * t810 * t322 - f64x8::splat(16.0) * t414 * t569;
            let t1654 = ((t39).select(-f64x8::splat(154.0) / f64x8::splat(81.0) * t41 * t201 * t518 - f64x8::splat(418.0) / f64x8::splat(243.0) * t59 * t205 * t523 - f64x8::splat(15.0) / f64x8::splat(4.0) * t78 * t60 * t527 - f64x8::splat(665.0) / f64x8::splat(1458.0) * t96 * t212 * t533, t1413 + t1449 + t1484 + t1514 + t1547 + t1582 + t1614 + t1650));
            let t1659 = ((t2).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(45.0) * t7 * t507 * t240 - t7 * t247 * t426 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(10.0) * t7 * t131 * t822 + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t1654));
            let tv4rho3sigma0 = f64x8::splat(2.0) * v_rho * t1659 + f64x8::splat(6.0) * t827;
            acc_v4rho3sigma = tv4rho3sigma0;
            let t1679 = t446 * t276;
            let t1680 = t1679 * t118;
            let t1694 = ((t106).select(-f64x8::splat(7.0) / f64x8::splat(108.0) * t29 * t459 * t290, f64x8::splat(0.0)));
            let t1695 = t114 * t1694;
            let t1708 = t69 * t463;
            let t1716 = t383 * t383;
            let t1721 = f64x8::splat(576.0) * t1328 * t1680 - f64x8::splat(512.0) * t1334 * t1680 - f64x8::splat(4.0) * t901 * t703 - f64x8::splat(2.0) * t698 * t464 - f64x8::splat(4.0) * t367 * t906 - f64x8::splat(2.0) * t159 * t1695 + f64x8::splat(2.0) * t174 * t177 * t1694 + f64x8::splat(4.0) * t181 * t1694 * t118 * t121 + f64x8::splat(96.0) * t653 * t479 * t294 + f64x8::splat(96.0) * t1708 * t1221 - f64x8::splat(96.0) * t592 * t417 * t383 - f64x8::splat(144.0) * t1339 * t1680 + f64x8::splat(20.0) / f64x8::splat(3.0) * t1716 - f64x8::splat(4.0) * param_mu * t1716 * t114;
            let t1724 = t463 * t276;
            let t1729 = t446 * t294;
            let t1732 = t222 * t693;
            let t1735 = t165 * t888;
            let t1738 = t294 * t463;
            let t1741 = param_alpha * t888;
            let t1753 = f64x8::splat(120.0) * t1228 * t1679 - f64x8::splat(24.0) * t592 * t1724 - f64x8::splat(32.0) * t414 * t709 - f64x8::splat(24.0) * t592 * t1729 + f64x8::splat(12.0) * t334 * t1732 + f64x8::splat(12.0) * t334 * t1735 + f64x8::splat(6.0) * t334 * t1738 - f64x8::splat(32.0) * t1741 * t182 - f64x8::splat(16.0) * t963 * t322 + f64x8::splat(20.0) / f64x8::splat(3.0) * t1732 + f64x8::splat(10.0) / f64x8::splat(3.0) * t1738 + f64x8::splat(20.0) / f64x8::splat(3.0) * t1735 + f64x8::splat(10.0) / f64x8::splat(3.0) * t107 * t1694 - f64x8::splat(144.0) * t1406 * t771;
            let t1764 = t901 * t588;
            let t1770 = t281 * t936;
            let t1774 = t182 * t121 * t383;
            let t1777 = t109 * t69;
            let t1778 = t1679 * t114;
            let t1781 = t1724 * t177;
            let t1788 = t391 * t113 * t693;
            let t1791 = t1729 * t114;
            let t1796 = f64x8::splat(192.0) * t1395 * t311 * t881 - f64x8::splat(8.0) * t305 * t856 * t787 - f64x8::splat(288.0) * t765 * t397 * t688 + f64x8::splat(672.0) * t1252 * t1770 - f64x8::splat(16.0) * t1305 * t1778 - f64x8::splat(288.0) * t1399 * t868 + f64x8::splat(32.0) * t1410 * t771 + f64x8::splat(96.0) * t1414 * t775 + f64x8::splat(96.0) * t1764 * t771 - f64x8::splat(576.0) * t740 * t1774 - f64x8::splat(156.0) * t1777 * t1778 + f64x8::splat(24.0) * t623 * t1781 - f64x8::splat(8.0) * t301 * t1788 + f64x8::splat(8.0) * t619 * t1791;
            let t1797 = t1679 * t177;
            let t1800 = t450 * t787;
            let t1823 = t646 * t850;
            let t1826 = t281 * t850;
            let t1835 = f64x8::splat(48.0) * t627 * t450 * t310 * t294 - f64x8::splat(16.0) * t305 * t391 * t1570 + f64x8::splat(24.0) * t796 * t599 * t850 + f64x8::splat(112.0) * t1261 * t1778 + f64x8::splat(336.0) * t1265 * t1797 - f64x8::splat(64.0) * t1309 * t1797 - f64x8::splat(192.0) * t1313 * t1770 - f64x8::splat(72.0) * t609 * t1781 - f64x8::splat(36.0) * t604 * t1791 - f64x8::splat(72.0) * t609 * t1800 + f64x8::splat(24.0) * t623 * t1800 + f64x8::splat(10.0) * t280 * t1823 - f64x8::splat(4.0) * t301 * t1823 - f64x8::splat(36.0) * t604 * t1826 + f64x8::splat(8.0) * t619 * t1826;
            let t1838 = t888 * t111;
            let t1839 = t1838 * t1466;
            let t1879 = f64x8::splat(20.0) * t280 * t1839 - f64x8::splat(8.0) * t301 * t1839 - f64x8::splat(16.0) * t305 * t1838 * t392 + f64x8::splat(192.0) * t730 * t731 * t688 - f64x8::splat(384.0) * t1277 * t281 * t950 + f64x8::splat(48.0) * t627 * t281 * t954 + f64x8::splat(96.0) * t720 * t1774 + f64x8::splat(256.0) * t748 * t1774 + f64x8::splat(96.0) * t54 * t222 * t111 * t929 * t688 - f64x8::splat(312.0) * t927 * param_alpha * t108 * t276 * t177 + f64x8::splat(24.0) * t875 * t1145 + f64x8::splat(20.0) * t280 * t1788 - f64x8::splat(16.0) * t308 * t311 * t1716 + f64x8::splat(24.0) * t330 * t480;
            let t1881 = t1716 * t118 * t121;
            let t1888 = t177 * t1716;
            let t1898 = t575 * t463;
            let t1912 = t310 * t108;
            let t1918 = -f64x8::splat(312.0) * t89 * t446 * t111 * t1912 * t276 - f64x8::splat(32.0) * param_alpha * t1716 * t118 - f64x8::splat(256.0) * t1212 * t446 * t319 + f64x8::splat(24.0) * t110 * t1881 + f64x8::splat(24.0) * t928 * t1240 - f64x8::splat(2.0) * t187 * t1694 + f64x8::splat(12.0) * t334 * t1716 - f64x8::splat(32.0) * t327 * t1881 + f64x8::splat(20.0) * t285 * t1888 + f64x8::splat(64.0) * t1898 * t319 + f64x8::splat(24.0) * t875 * t282 + f64x8::splat(24.0) * t927 * t286 + f64x8::splat(64.0) * t924 * t323 - f64x8::splat(32.0) * t411 * t710;
            let t1921 = t1716 * t111 * t113;
            let t1929 = t54 * t107 * t111;
            let t1962 = f64x8::splat(20.0) * t375 * t176 * t888 * t165 - f64x8::splat(816.0) * t1182 * t446 * t319 + f64x8::splat(672.0) * t1552 * t950 * t276 + f64x8::splat(24.0) * t1929 * t940 * t276 - f64x8::splat(72.0) * t765 * t954 * t276 + f64x8::splat(10.0) * t375 * t940 * t294 - f64x8::splat(72.0) * t765 * t936 * t294 + f64x8::splat(24.0) * t588 * t463 * t319 + f64x8::splat(20.0) * t375 * t376 * t693 + f64x8::splat(2.0) * t171 * t1695 - f64x8::splat(16.0) * t305 * t1888 + f64x8::splat(20.0) * t280 * t1921 - f64x8::splat(8.0) * t301 * t1921 + f64x8::splat(24.0) * t404 * t710;
            let t2002 = t327 * t888;
            let t2007 = -f64x8::splat(384.0) * t1632 * t1195 * t446 * t276 - f64x8::splat(16.0) * t396 * t310 * t888 * t165 + f64x8::splat(48.0) * t730 * t555 * t463 * t276 + f64x8::splat(24.0) * t110 * t888 * t183 + f64x8::splat(1152.0) * t1203 * t446 * t319 + f64x8::splat(384.0) * t1607 * t182 * t383 - f64x8::splat(8.0) * t396 * t954 * t294 + f64x8::splat(48.0) * t730 * t950 * t294 - f64x8::splat(144.0) * t564 * t463 * t319 - f64x8::splat(16.0) * t396 * t397 * t693 - f64x8::splat(32.0) * t2002 * t183 - f64x8::splat(144.0) * t913 * t323 + f64x8::splat(12.0) * t916 * t323 - f64x8::splat(16.0) * t919 * t323 + f64x8::splat(24.0) * t958 * t323;
            let t2011 = ((t39).select(f64x8::splat(19.0) / f64x8::splat(81.0) * t56 * t434 * t261 + f64x8::splat(3.0) / f64x8::splat(4.0) * t78 * v_sigma * t265 + f64x8::splat(35.0) / f64x8::splat(324.0) * t96 * t441 * t271, t1721 + t1753 + t1796 + t1835 + t1879 + t1918 + t1962 + t2007));
            let t2016 = ((t2).select(f64x8::splat(0.0), -t7 * t247 * t500 / f64x8::splat(30.0) + t7 * t131 * t977 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t2011));
            let tv4rho2sigma20 = f64x8::splat(2.0) * v_rho * t2016 + f64x8::splat(4.0) * t982;
            acc_v4rho2sigma2 = tv4rho2sigma20;
            let t2033 = ((t106).select(-t29 * t1003 * t161 / f64x8::splat(24.0), f64x8::splat(0.0)));
            let t2034 = t114 * t2033;
            let t2044 = t1011 * t165;
            let t2062 = f64x8::splat(4.0) * t181 * t2033 * t118 * t121 - f64x8::splat(72.0) * t592 * t999 * t165 + f64x8::splat(2.0) * t174 * t177 * t2033 - f64x8::splat(6.0) * param_mu * t383 * t464 + f64x8::splat(288.0) * t653 * t479 * t383 + f64x8::splat(24.0) * t1394 * t1012 - f64x8::splat(2.0) * t367 * t1023 + f64x8::splat(576.0) * t1328 * t2044 - f64x8::splat(512.0) * t1334 * t2044 - f64x8::splat(144.0) * t1339 * t2044 - f64x8::splat(2.0) * t159 * t2034;
            let t2071 = t1010 * t165;
            let t2077 = t383 * t463;
            let t2080 = t222 * t888;
            let t2083 = param_alpha * t1007;
            let t2086 = t165 * t1007;
            let t2091 = -f64x8::splat(6.0) * t901 * t906 + f64x8::splat(24.0) * t1017 * t178 - f64x8::splat(48.0) * t1741 * t231 - f64x8::splat(48.0) * t963 * t407 + f64x8::splat(120.0) * t1228 * t2071 - f64x8::splat(72.0) * t592 * t446 * t383 + f64x8::splat(18.0) * t334 * t2077 + f64x8::splat(18.0) * t334 * t2080 - f64x8::splat(16.0) * t2083 * t182 + f64x8::splat(6.0) * t334 * t2086 + f64x8::splat(10.0) / f64x8::splat(3.0) * t107 * t2033;
            let t2119 = f64x8::splat(10.0) * t2077 + f64x8::splat(10.0) * t2080 + f64x8::splat(10.0) / f64x8::splat(3.0) * t2086 + f64x8::splat(144.0) * t626 * t963 * t311 * t417 + f64x8::splat(72.0) * t1764 * t851 - f64x8::splat(108.0) * t1406 * t851 - f64x8::splat(216.0) * t174 * t414 * t857 + f64x8::splat(24.0) * t618 * t85 * t463 * t372 + f64x8::splat(72.0) * t622 * t1708 * t393 + f64x8::splat(72.0) * t875 * t1475 + f64x8::splat(72.0) * t1929 * t376 * t883;
            let t2123 = t1007 * t111;
            let t2124 = t2123 * t1466;
            let t2132 = t450 * t860;
            let t2135 = t2071 * t114;
            let t2138 = t1049 * t392;
            let t2141 = t450 * t758;
            let t2144 = t1049 * t630;
            let t2154 = -f64x8::splat(384.0) * t1277 * t1049 * t555 * t165 - f64x8::splat(8.0) * t305 * t2123 * t392 - f64x8::splat(216.0) * t765 * t397 * t883 + f64x8::splat(144.0) * t627 * t450 * t723 - f64x8::splat(16.0) * t1305 * t2135 - f64x8::splat(64.0) * t1309 * t2138 - f64x8::splat(192.0) * t1313 * t2144 + f64x8::splat(10.0) * t280 * t2124 - f64x8::splat(4.0) * t301 * t2124 + f64x8::splat(24.0) * t619 * t2132 + f64x8::splat(72.0) * t623 * t2141;
            let t2157 = t2080 * t114;
            let t2160 = t2077 * t114;
            let t2169 = t1320 * t883;
            let t2185 = f64x8::splat(192.0) * t1898 * t417 * t1320 - f64x8::splat(24.0) * t305 * t1838 * t376 - f64x8::splat(24.0) * t305 * t856 * t758 + f64x8::splat(112.0) * t1261 * t2135 + f64x8::splat(336.0) * t1265 * t2138 - f64x8::splat(156.0) * t1777 * t2135 - f64x8::splat(108.0) * t604 * t2132 - f64x8::splat(12.0) * t301 * t2157 - f64x8::splat(12.0) * t301 * t2160 + f64x8::splat(72.0) * t720 * t2169 - f64x8::splat(432.0) * t740 * t2169;
            let t2186 = t285 * param_alpha;
            let t2214 = -f64x8::splat(8.0) * t396 * t310 * t1007 * t165 + f64x8::splat(12.0) * t110 * t1007 * t183 + f64x8::splat(1152.0) * t1203 * t1010 * t183 + f64x8::splat(144.0) * t730 * t731 * t883 + f64x8::splat(672.0) * t1252 * t2144 - f64x8::splat(2.0) * t187 * t2033 - f64x8::splat(312.0) * t2186 * t2138 - f64x8::splat(216.0) * t609 * t2141 + f64x8::splat(30.0) * t280 * t2157 + f64x8::splat(30.0) * t280 * t2160 - f64x8::splat(432.0) * t913 * t408;
            let t2216 = t327 * t1007;
            let t2250 = -f64x8::splat(312.0) * t89 * t1010 * t111 * t1912 * t165 + f64x8::splat(10.0) * t375 * t176 * t1007 * t165 + f64x8::splat(672.0) * t1552 * t555 * t1010 * t165 - f64x8::splat(256.0) * t1212 * t1010 * t183 - f64x8::splat(216.0) * t765 * t936 * t383 - f64x8::splat(24.0) * t396 * t397 * t888 - f64x8::splat(24.0) * t396 * t723 * t463 + f64x8::splat(24.0) * t1071 * t166 + f64x8::splat(72.0) * t928 * t1615 - f64x8::splat(16.0) * t2216 * t183 + f64x8::splat(192.0) * t924 * t408;
            let t2283 = -f64x8::splat(384.0) * t1632 * t1195 * t1010 * t165 + f64x8::splat(288.0) * t1708 * t169 * t182 * t222 - f64x8::splat(816.0) * t1182 * t1010 * t183 + f64x8::splat(30.0) * t375 * t376 * t888 + f64x8::splat(30.0) * t375 * t758 * t463 + f64x8::splat(144.0) * t730 * t950 * t383 + f64x8::splat(2.0) * t171 * t2034 - f64x8::splat(48.0) * t2002 * t232 + f64x8::splat(36.0) * t404 * t895 - f64x8::splat(48.0) * t919 * t408 + f64x8::splat(72.0) * t958 * t408 + f64x8::splat(36.0) * t735 * t484;
            let t2287 = ((t39).select(-t78 * t148 / f64x8::splat(12.0) - t96 * t46 * t154 / f64x8::splat(54.0), t2062 + t2091 + t2119 + t2154 + t2185 + t2214 + t2250 + t2283));
            let t2292 = ((t2).select(f64x8::splat(0.0), t7 * t131 * t1082 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t2287));
            let tv4rhosigma30 = f64x8::splat(2.0) * v_rho * t2292 + f64x8::splat(2.0) * t1086;
            acc_v4rhosigma3 = tv4rhosigma30;
            let t2305 = ((t106).select(-f64x8::splat(5.0) / f64x8::splat(64.0) * t29 / t30 / t79 * t31 * t34, f64x8::splat(0.0)));
            let t2308 = t463 * t463;
            let t2314 = t446 * t446;
            let t2322 = t2308 * t118 * t121;
            let t2325 = t177 * t2308;
            let t2334 = t2314 * t118;
            let t2335 = t2334 * t121;
            let t2349 = t556 * t2314;
            let t2352 = f64x8::splat(4.0) * t181 * t2305 * t118 * t121 + f64x8::splat(576.0) * t1708 * t169 * t446 * t118 + f64x8::splat(24.0) * param_mu * t2314 * t1171 - f64x8::splat(48.0) * param_alpha * t2308 * t118 - f64x8::splat(312.0) * t89 * t2314 * t1216 - f64x8::splat(8.0) * t901 * t1023 + f64x8::splat(36.0) * t110 * t2322 + f64x8::splat(672.0) * t1178 * t2349 - f64x8::splat(816.0) * t1182 * t2335 - f64x8::splat(256.0) * t1212 * t2335 + f64x8::splat(120.0) * t1228 * t2314 - f64x8::splat(2.0) * t187 * t2305 + f64x8::splat(18.0) * t334 * t2308 - f64x8::splat(48.0) * t327 * t2322 + f64x8::splat(30.0) * t285 * t2325 + f64x8::splat(10.0) * t2308;
            let t2353 = t114 * t2305;
            let t2367 = t446 * t463;
            let t2372 = t222 * t1007;
            let t2393 = -f64x8::splat(2.0) * t159 * t2353 - f64x8::splat(384.0) * t1193 * t1196 * t2314 + f64x8::splat(2.0) * t174 * t177 * t2305 + f64x8::splat(1152.0) * t1203 * t2335 - f64x8::splat(24.0) * t308 * t311 * t2308 - f64x8::splat(144.0) * t592 * t2367 - f64x8::splat(144.0) * t1339 * t2334 + f64x8::splat(24.0) * t334 * t2372 - f64x8::splat(64.0) * t2083 * t231 - f64x8::splat(512.0) * t1334 * t2334 + f64x8::splat(576.0) * t1328 * t2334 + f64x8::splat(24.0) * param_alpha * t2314 * t1320 - f64x8::splat(6.0) * param_mu * t2308 * t114 + f64x8::splat(24.0) * t54 * t2314 * t177 + f64x8::splat(40.0) / f64x8::splat(3.0) * t2372 + f64x8::splat(10.0) / f64x8::splat(3.0) * t107 * t2305;
            let t2395 = t2367 * t114;
            let t2398 = t2367 * t177;
            let t2408 = t391 * t113 * t1007;
            let t2434 = t2314 * t111;
            let t2435 = t2434 * t113;
            let t2438 = f64x8::splat(48.0) * t619 * t2395 + f64x8::splat(144.0) * t623 * t2398 - f64x8::splat(32.0) * t305 * t2123 * t376 + f64x8::splat(288.0) * t627 * t856 * t936 + f64x8::splat(40.0) * t280 * t2408 - f64x8::splat(216.0) * t604 * t2395 - f64x8::splat(432.0) * t609 * t2398 - f64x8::splat(16.0) * t301 * t2408 + f64x8::splat(144.0) * t159 * param_alpha * t2395 - f64x8::splat(384.0) * t1277 * t2349 - f64x8::splat(32.0) * t396 * t397 * t1007 + f64x8::splat(288.0) * t730 * t950 * t463 - f64x8::splat(864.0) * t913 * t484 - f64x8::splat(64.0) * t2216 * t232 + f64x8::splat(384.0) * t1898 * t480 + f64x8::splat(112.0) * t1261 * t2435;
            let t2439 = t2434 * t176;
            let t2442 = t2434 * t310;
            let t2474 = t2308 * t111 * t113;
            let t2479 = f64x8::splat(144.0) * t1929 * t176 * t446 * t463 + f64x8::splat(40.0) * t375 * t376 * t1007 - f64x8::splat(432.0) * t765 * t936 * t463 + f64x8::splat(48.0) * t404 * t1035 + f64x8::splat(672.0) * t1252 * t2442 + f64x8::splat(336.0) * t1265 * t2439 - f64x8::splat(16.0) * t1305 * t2435 - f64x8::splat(64.0) * t1309 * t2439 - f64x8::splat(192.0) * t1313 * t2442 + f64x8::splat(2.0) * t171 * t2353 - f64x8::splat(156.0) * t1777 * t2435 - f64x8::splat(312.0) * t2186 * t2439 - f64x8::splat(24.0) * t305 * t2325 + f64x8::splat(30.0) * t280 * t2474 - f64x8::splat(12.0) * t301 * t2474 + f64x8::splat(144.0) * t958 * t484;
            let t2482 = ((t39).select(t93 * t95 * t45 * t101 / f64x8::splat(576.0), t2352 + t2393 + t2438 + t2479));
            let t2486 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t2482));
            let tv4sigma40 = f64x8::splat(2.0) * v_rho * t2486;
            acc_v4sigma4 = tv4sigma40;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        store_add(v3rho3, ip, m, acc_v3rho3);
        store_add(v3rho2sigma, ip, m, acc_v3rho2sigma);
        store_add(v3rhosigma2, ip, m, acc_v3rhosigma2);
        store_add(v3sigma3, ip, m, acc_v3sigma3);
        store_add(v4rho4, ip, m, acc_v4rho4);
        store_add(v4rho3sigma, ip, m, acc_v4rho3sigma);
        store_add(v4rho2sigma2, ip, m, acc_v4rho2sigma2);
        store_add(v4rhosigma3, ip, m, acc_v4rhosigma3);
        store_add(v4sigma4, ip, m, acc_v4sigma4);
        ip += 8;
    }
}
