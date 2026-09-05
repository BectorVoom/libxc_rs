//! GGA_X_PBETRANS kxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbetrans.c`
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
pub fn gga_x_pbetrans_kxc_unpol(
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
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = (simd::cbrt(v_rho));
            let t19 = t17 * t18;
            let t20 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t21 = (simd::cbrt(t20));
            let t23 = f64x8::splat(M_CBRT6);
            let t24 = t23 * t23;
            let t27 = ((v_sigma).sqrt());
            let t28 = f64x8::splat(M_CBRT2);
            let t29 = t27 * t28;
            let t31 = f64x8::splat(1.0) / t18 / v_rho;
            let t38 = (simd::exp(-f64x8::splat(2.0) * t3 * t21 * (t24 / t21 * t29 * t31 / f64x8::splat(12.0) - f64x8::splat(3.0))));
            let t39 = f64x8::splat(1.0) + t38;
            let t41 = f64x8::splat(0.413) / t39;
            let t42 = f64x8::splat(1.227) - t41;
            let t43 = t21 * t21;
            let t45 = t23 / t43;
            let t46 = t28 * t28;
            let t47 = v_sigma * t46;
            let t48 = v_rho * v_rho;
            let t49 = t18 * t18;
            let t51 = f64x8::splat(1.0) / t49 / t48;
            let t55 = f64x8::splat(1.227) - t41 + f64x8::splat(0.009125) * t45 * t47 * t51;
            let t56 = f64x8::splat(1.0) / t55;
            let t58 = -t42 * t56 + f64x8::splat(1.0);
            let t60 = t42 * t58 + f64x8::splat(1.0);
            let t64 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t60));
            let tzk0 = f64x8::splat(2.0) * t64;
            acc_zk = tzk0;
            let t66 = t17 / t49;
            let t70 = t39 * t39;
            let t71 = f64x8::splat(1.0) / t70;
            let t72 = t71 * t3;
            let t73 = t24 * t27;
            let t74 = t72 * t73;
            let t76 = f64x8::splat(1.0) / t18 / t48;
            let t77 = t28 * t76;
            let t78 = t38 * t58;
            let t79 = t77 * t78;
            let t82 = t38 * t56;
            let t83 = t77 * t82;
            let t86 = t55 * t55;
            let t87 = f64x8::splat(1.0) / t86;
            let t88 = t42 * t87;
            let t89 = t72 * t24;
            let t90 = t76 * t38;
            let t94 = t48 * v_rho;
            let t96 = f64x8::splat(1.0) / t49 / t94;
            let t100 = f64x8::splat(0.09177777777777778) * t89 * t29 * t90 - f64x8::splat(0.024333333333333332) * t45 * t47 * t96;
            let t102 = -f64x8::splat(0.09177777777777778) * t74 * t83 + t88 * t100;
            let t104 = f64x8::splat(0.09177777777777778) * t74 * t79 + t42 * t102;
            let t109 = ((t2).select(f64x8::splat(0.0), -t6 * t66 * t60 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t104));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t109 + f64x8::splat(2.0) * t64;
            acc_vrho = tvrho0;
            let t112 = f64x8::splat(1.0) / t27;
            let t113 = t24 * t112;
            let t114 = t72 * t113;
            let t115 = t28 * t31;
            let t116 = t115 * t78;
            let t119 = t115 * t82;
            let t122 = t112 * t28;
            let t123 = t31 * t38;
            let t127 = t46 * t51;
            let t130 = -f64x8::splat(0.034416666666666665) * t89 * t122 * t123 + f64x8::splat(0.009125) * t45 * t127;
            let t132 = f64x8::splat(0.034416666666666665) * t114 * t119 + t88 * t130;
            let t134 = -f64x8::splat(0.034416666666666665) * t114 * t116 + t42 * t132;
            let t138 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t134));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t138;
            acc_vsigma = tvsigma0;
            let t143 = t17 / t49 / v_rho;
            let t151 = f64x8::splat(1.0) / t70 / t39;
            let t152 = t3 * t3;
            let t153 = t151 * t152;
            let t154 = t23 * v_sigma;
            let t155 = t153 * t154;
            let t156 = t48 * t48;
            let t158 = f64x8::splat(1.0) / t49 / t156;
            let t159 = t46 * t158;
            let t160 = t38 * t38;
            let t161 = t160 * t58;
            let t162 = t159 * t161;
            let t166 = f64x8::splat(1.0) / t18 / t94;
            let t167 = t28 * t166;
            let t168 = t167 * t78;
            let t171 = t71 * t152;
            let t172 = t171 * t154;
            let t173 = t159 * t78;
            let t176 = t38 * t102;
            let t177 = t77 * t176;
            let t180 = t160 * t56;
            let t181 = t159 * t180;
            let t184 = t167 * t82;
            let t187 = t159 * t82;
            let t190 = t38 * t87;
            let t191 = t190 * t100;
            let t192 = t77 * t191;
            let t196 = f64x8::splat(1.0) / t86 / t55;
            let t197 = t42 * t196;
            let t198 = t100 * t100;
            let t201 = t153 * t23;
            let t202 = t158 * t160;
            let t206 = t166 * t38;
            let t210 = t171 * t23;
            let t211 = t158 * t38;
            let t218 = -f64x8::splat(0.24474074074074073) * t201 * t47 * t202 - f64x8::splat(0.21414814814814814) * t89 * t29 * t206 + f64x8::splat(0.12237037037037037) * t210 * t47 * t211 + f64x8::splat(0.08922222222222222) * t45 * t47 * t158;
            let t220 = f64x8::splat(0.24474074074074073) * t155 * t181 + f64x8::splat(0.21414814814814814) * t74 * t184 - f64x8::splat(0.12237037037037037) * t172 * t187 + f64x8::splat(0.18355555555555556) * t74 * t192 - f64x8::splat(2.0) * t197 * t198 + t88 * t218;
            let t222 = -f64x8::splat(0.24474074074074073) * t155 * t162 - f64x8::splat(0.21414814814814814) * t74 * t168 + f64x8::splat(0.12237037037037037) * t172 * t173 + f64x8::splat(0.18355555555555556) * t74 * t177 + t42 * t220;
            let t227 = ((t2).select(f64x8::splat(0.0), t6 * t143 * t60 / f64x8::splat(12.0) - t6 * t66 * t104 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t222));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t227 + f64x8::splat(4.0) * t109;
            acc_v2rho2 = tv2rho20;
            let t233 = t46 * t96;
            let t234 = t233 * t161;
            let t239 = t233 * t78;
            let t242 = t115 * t176;
            let t245 = t38 * t132;
            let t246 = t77 * t245;
            let t249 = t233 * t180;
            let t254 = t233 * t82;
            let t257 = t115 * t191;
            let t260 = t190 * t130;
            let t261 = t77 * t260;
            let t264 = t130 * t100;
            let t278 = f64x8::splat(0.09177777777777778) * t201 * t233 * t160 + f64x8::splat(0.04588888888888889) * t89 * t122 * t90 - f64x8::splat(0.04588888888888889) * t210 * t233 * t38 - f64x8::splat(0.024333333333333332) * t45 * t233;
            let t280 = -f64x8::splat(0.09177777777777778) * t201 * t249 - f64x8::splat(0.04588888888888889) * t114 * t83 + f64x8::splat(0.04588888888888889) * t210 * t254 - f64x8::splat(0.034416666666666665) * t114 * t257 + f64x8::splat(0.09177777777777778) * t74 * t261 - f64x8::splat(2.0) * t197 * t264 + t88 * t278;
            let t282 = f64x8::splat(0.09177777777777778) * t201 * t234 + f64x8::splat(0.04588888888888889) * t114 * t79 - f64x8::splat(0.04588888888888889) * t210 * t239 - f64x8::splat(0.034416666666666665) * t114 * t242 + f64x8::splat(0.09177777777777778) * t74 * t246 + t42 * t280;
            let t287 = ((t2).select(f64x8::splat(0.0), -t6 * t66 * t134 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t282));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t287 + f64x8::splat(2.0) * t138;
            acc_v2rhosigma = tv2rhosigma0;
            let t290 = f64x8::splat(1.0) / v_sigma;
            let t291 = t23 * t290;
            let t292 = t153 * t291;
            let t293 = t127 * t161;
            let t296 = t27 * v_sigma;
            let t297 = f64x8::splat(1.0) / t296;
            let t298 = t24 * t297;
            let t299 = t72 * t298;
            let t302 = t171 * t291;
            let t303 = t127 * t78;
            let t306 = t115 * t245;
            let t309 = t127 * t180;
            let t314 = t127 * t82;
            let t317 = t115 * t260;
            let t320 = t130 * t130;
            let t323 = t290 * t46;
            let t324 = t51 * t160;
            let t328 = t297 * t28;
            let t332 = t51 * t38;
            let t336 = -f64x8::splat(0.034416666666666665) * t201 * t323 * t324 + f64x8::splat(0.017208333333333332) * t89 * t328 * t123 + f64x8::splat(0.017208333333333332) * t210 * t323 * t332;
            let t338 = f64x8::splat(0.034416666666666665) * t292 * t309 - f64x8::splat(0.017208333333333332) * t299 * t119 - f64x8::splat(0.017208333333333332) * t302 * t314 - f64x8::splat(0.06883333333333333) * t114 * t317 - f64x8::splat(2.0) * t197 * t320 + t88 * t336;
            let t340 = -f64x8::splat(0.034416666666666665) * t292 * t293 + f64x8::splat(0.017208333333333332) * t299 * t116 + f64x8::splat(0.017208333333333332) * t302 * t303 - f64x8::splat(0.06883333333333333) * t114 * t306 + t42 * t338;
            let t344 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t340));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t344;
            acc_v2sigma2 = tv2sigma20;
            let t347 = t17 * t51;
            let t357 = t70 * t70;
            let t358 = f64x8::splat(1.0) / t357;
            let t359 = t358 * t296;
            let t360 = t156 * t94;
            let t361 = f64x8::splat(1.0) / t360;
            let t362 = t160 * t38;
            let t363 = t361 * t362;
            let t367 = t156 * v_rho;
            let t369 = f64x8::splat(1.0) / t49 / t367;
            let t370 = t46 * t369;
            let t371 = t370 * t161;
            let t374 = t151 * t296;
            let t375 = t361 * t160;
            let t379 = t160 * t102;
            let t380 = t159 * t379;
            let t384 = f64x8::splat(1.0) / t18 / t156;
            let t385 = t28 * t384;
            let t386 = t385 * t78;
            let t389 = t370 * t78;
            let t392 = t167 * t176;
            let t395 = t71 * t296;
            let t396 = t361 * t38;
            let t400 = t159 * t176;
            let t403 = t38 * t220;
            let t404 = t77 * t403;
            let t410 = t370 * t180;
            let t416 = t160 * t87;
            let t417 = t416 * t100;
            let t421 = t385 * t82;
            let t424 = t370 * t82;
            let t427 = t167 * t191;
            let t436 = t38 * t196;
            let t437 = t436 * t198;
            let t438 = t77 * t437;
            let t441 = t190 * t218;
            let t442 = t77 * t441;
            let t445 = t86 * t86;
            let t446 = f64x8::splat(1.0) / t445;
            let t447 = t42 * t446;
            let t448 = t198 * t100;
            let t462 = t384 * t38;
            let t475 = f64x8::splat(5.873777777777778) * t359 * t363 + f64x8::splat(1.7131851851851851) * t201 * t47 * t369 * t160 - f64x8::splat(5.873777777777778) * t374 * t375 + f64x8::splat(0.7138271604938271) * t89 * t29 * t462 - f64x8::splat(0.8565925925925926) * t210 * t47 * t369 * t38 + f64x8::splat(0.9789629629629629) * t395 * t396 - f64x8::splat(0.4163703703703704) * t45 * t47 * t369;
            let t477 = -f64x8::splat(5.873777777777778) * t359 * t363 * t56 - f64x8::splat(1.7131851851851851) * t155 * t410 + f64x8::splat(5.873777777777778) * t374 * t375 * t56 - f64x8::splat(0.7342222222222222) * t155 * t159 * t417 - f64x8::splat(0.7138271604938271) * t74 * t421 + f64x8::splat(0.8565925925925926) * t172 * t424 - f64x8::splat(0.6424444444444445) * t74 * t427 - f64x8::splat(0.9789629629629629) * t395 * t396 * t56 + f64x8::splat(0.3671111111111111) * t172 * t159 * t191 - f64x8::splat(0.5506666666666666) * t74 * t438 + f64x8::splat(0.2753333333333333) * t74 * t442 + f64x8::splat(6.0) * t447 * t448 - f64x8::splat(6.0) * t197 * t100 * t218 + t88 * t475;
            let t479 = f64x8::splat(5.873777777777778) * t359 * t363 * t58 + f64x8::splat(1.7131851851851851) * t155 * t371 - f64x8::splat(5.873777777777778) * t374 * t375 * t58 - f64x8::splat(0.7342222222222222) * t155 * t380 + f64x8::splat(0.7138271604938271) * t74 * t386 - f64x8::splat(0.8565925925925926) * t172 * t389 - f64x8::splat(0.6424444444444445) * t74 * t392 + f64x8::splat(0.9789629629629629) * t395 * t396 * t58 + f64x8::splat(0.3671111111111111) * t172 * t400 + f64x8::splat(0.2753333333333333) * t74 * t404 + t42 * t477;
            let t484 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t347 * t60 + t6 * t143 * t104 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t66 * t222 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t479));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t484 + f64x8::splat(6.0) * t227;
            acc_v3rho3 = tv3rho30;
            let t494 = t156 * t48;
            let t495 = f64x8::splat(1.0) / t494;
            let t496 = t358 * t495;
            let t497 = t362 * t58;
            let t498 = t497 * t27;
            let t503 = t151 * t495;
            let t504 = t161 * t27;
            let t507 = t233 * t379;
            let t516 = t71 * t495;
            let t517 = t27 * t38;
            let t518 = t517 * t58;
            let t521 = t233 * t176;
            let t524 = t115 * t403;
            let t527 = t160 * t132;
            let t528 = t159 * t527;
            let t531 = t167 * t245;
            let t534 = t159 * t245;
            let t537 = t38 * t280;
            let t538 = t77 * t537;
            let t541 = t190 * t278;
            let t542 = t77 * t541;
            let t545 = t416 * t130;
            let t556 = t115 * t441;
            let t559 = t362 * t56;
            let t560 = t559 * t27;
            let t563 = t180 * t27;
            let t566 = t517 * t56;
            let t572 = t23 * t46;
            let t573 = t171 * t572;
            let t574 = t96 * t38;
            let t575 = t87 * t100;
            let t581 = t153 * t572;
            let t582 = t96 * t160;
            let t586 = t167 * t260;
            let t590 = t72 * t73 * t28;
            let t591 = t196 * t130;
            let t592 = t591 * t100;
            let t593 = t90 * t592;
            let t596 = t115 * t437;
            let t607 = t362 * t27;
            let t613 = t160 * t27;
            let t626 = -f64x8::splat(2.2026666666666666) * t496 * t607 - f64x8::splat(0.4588888888888889) * t201 * t159 * t160 + f64x8::splat(2.2026666666666666) * t503 * t613 - f64x8::splat(0.10707407407407407) * t89 * t122 * t206 + f64x8::splat(0.22944444444444445) * t210 * t159 * t38 - f64x8::splat(0.3671111111111111) * t516 * t517 + f64x8::splat(0.08922222222222222) * t45 * t159;
            let t628 = f64x8::splat(0.18355555555555556) * t74 * t542 - f64x8::splat(0.24474074074074073) * t155 * t159 * t545 + f64x8::splat(0.12237037037037037) * t172 * t159 * t260 + f64x8::splat(0.10707407407407407) * t114 * t184 + f64x8::splat(0.09177777777777778) * t114 * t192 - f64x8::splat(0.034416666666666665) * t114 * t556 + f64x8::splat(2.2026666666666666) * t496 * t560 - f64x8::splat(2.2026666666666666) * t503 * t563 + f64x8::splat(0.3671111111111111) * t516 * t566 + f64x8::splat(6.0) * t447 * t130 * t198 - f64x8::splat(0.09177777777777778) * t573 * t574 * t575 + f64x8::splat(0.4588888888888889) * t201 * t181 + f64x8::splat(0.18355555555555556) * t581 * t582 * t575 - f64x8::splat(0.21414814814814814) * t74 * t586 - f64x8::splat(0.3671111111111111) * t590 * t593 + f64x8::splat(0.06883333333333333) * t114 * t596 - f64x8::splat(0.22944444444444445) * t210 * t187 - f64x8::splat(4.0) * t197 * t278 * t100 - f64x8::splat(2.0) * t197 * t130 * t218 + t88 * t626;
            let t630 = -f64x8::splat(2.2026666666666666) * t496 * t498 - f64x8::splat(0.4588888888888889) * t201 * t162 + f64x8::splat(2.2026666666666666) * t503 * t504 + f64x8::splat(0.18355555555555556) * t201 * t507 - f64x8::splat(0.10707407407407407) * t114 * t168 + f64x8::splat(0.22944444444444445) * t210 * t173 + f64x8::splat(0.09177777777777778) * t114 * t177 - f64x8::splat(0.3671111111111111) * t516 * t518 - f64x8::splat(0.09177777777777778) * t210 * t521 - f64x8::splat(0.034416666666666665) * t114 * t524 - f64x8::splat(0.24474074074074073) * t155 * t528 - f64x8::splat(0.21414814814814814) * t74 * t531 + f64x8::splat(0.12237037037037037) * t172 * t534 + f64x8::splat(0.18355555555555556) * t74 * t538 + t42 * t628;
            let t635 = ((t2).select(f64x8::splat(0.0), t6 * t143 * t134 / f64x8::splat(12.0) - t6 * t66 * t282 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t630));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t635 + f64x8::splat(4.0) * t287;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t641 = t358 * t112;
            let t642 = f64x8::splat(1.0) / t367;
            let t643 = t642 * t362;
            let t649 = t151 * t112;
            let t650 = t642 * t160;
            let t651 = t650 * t58;
            let t654 = t127 * t379;
            let t663 = t71 * t112;
            let t664 = t642 * t38;
            let t668 = t127 * t176;
            let t671 = t233 * t527;
            let t676 = t233 * t245;
            let t679 = t115 * t537;
            let t682 = t38 * t338;
            let t683 = t77 * t682;
            let t691 = t650 * t56;
            let t694 = t127 * t417;
            let t706 = t127 * t191;
            let t709 = t87 * t130;
            let t719 = t72 * t113 * t28;
            let t720 = t123 * t592;
            let t723 = t115 * t541;
            let t726 = t436 * t320;
            let t727 = t77 * t726;
            let t736 = t190 * t336;
            let t737 = t77 * t736;
            let t758 = f64x8::splat(0.826) * t641 * t643 + f64x8::splat(0.04588888888888889) * t201 * t323 * t582 - f64x8::splat(0.826) * t649 * t650 - f64x8::splat(0.022944444444444444) * t89 * t328 * t90 - f64x8::splat(0.022944444444444444) * t210 * t323 * t574 + f64x8::splat(0.13766666666666666) * t663 * t664;
            let t760 = -f64x8::splat(0.826) * t641 * t643 * t56 - f64x8::splat(0.04588888888888889) * t292 * t249 + f64x8::splat(0.826) * t649 * t691 - f64x8::splat(0.034416666666666665) * t292 * t694 + f64x8::splat(0.022944444444444444) * t299 * t83 + f64x8::splat(0.022944444444444444) * t302 * t254 + f64x8::splat(0.017208333333333332) * t299 * t257 - f64x8::splat(0.13766666666666666) * t663 * t664 * t56 + f64x8::splat(0.017208333333333332) * t302 * t706 + f64x8::splat(0.18355555555555556) * t581 * t582 * t709 + f64x8::splat(0.09177777777777778) * t114 * t261 - f64x8::splat(0.09177777777777778) * t573 * t574 * t709 + f64x8::splat(0.13766666666666666) * t719 * t720 - f64x8::splat(0.06883333333333333) * t114 * t723 - f64x8::splat(0.18355555555555556) * t74 * t727 + f64x8::splat(6.0) * t447 * t320 * t100 - f64x8::splat(4.0) * t197 * t130 * t278 + f64x8::splat(0.09177777777777778) * t74 * t737 - f64x8::splat(2.0) * t197 * t336 * t100 + t88 * t758;
            let t762 = f64x8::splat(0.826) * t641 * t643 * t58 + f64x8::splat(0.04588888888888889) * t292 * t234 - f64x8::splat(0.826) * t649 * t651 - f64x8::splat(0.034416666666666665) * t292 * t654 - f64x8::splat(0.022944444444444444) * t299 * t79 - f64x8::splat(0.022944444444444444) * t302 * t239 + f64x8::splat(0.017208333333333332) * t299 * t242 + f64x8::splat(0.13766666666666666) * t663 * t664 * t58 + f64x8::splat(0.017208333333333332) * t302 * t668 + f64x8::splat(0.18355555555555556) * t201 * t671 + f64x8::splat(0.09177777777777778) * t114 * t246 - f64x8::splat(0.09177777777777778) * t210 * t676 - f64x8::splat(0.06883333333333333) * t114 * t679 + f64x8::splat(0.09177777777777778) * t74 * t683 + t42 * t760;
            let t767 = ((t2).select(f64x8::splat(0.0), -t6 * t66 * t340 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t762));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t767 + f64x8::splat(2.0) * t344;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t770 = t358 * t297;
            let t771 = f64x8::splat(1.0) / t156;
            let t772 = t771 * t362;
            let t773 = t772 * t58;
            let t776 = v_sigma * v_sigma;
            let t777 = f64x8::splat(1.0) / t776;
            let t778 = t23 * t777;
            let t779 = t153 * t778;
            let t782 = t151 * t297;
            let t783 = t771 * t160;
            let t784 = t783 * t58;
            let t787 = t127 * t527;
            let t791 = f64x8::splat(1.0) / t27 / t776;
            let t793 = t72 * t24 * t791;
            let t796 = t171 * t778;
            let t801 = t71 * t297;
            let t802 = t771 * t38;
            let t803 = t802 * t58;
            let t806 = t127 * t245;
            let t809 = t115 * t682;
            let t812 = t772 * t56;
            let t817 = t783 * t56;
            let t820 = t127 * t545;
            let t829 = t802 * t56;
            let t832 = t127 * t260;
            let t835 = t115 * t726;
            let t838 = t115 * t736;
            let t841 = t320 * t130;
            let t844 = t130 * t336;
            let t849 = t777 * t46;
            let t855 = t791 * t28;
            let t864 = -f64x8::splat(0.30975) * t770 * t772 + f64x8::splat(0.051625) * t201 * t849 * t324 + f64x8::splat(0.30975) * t782 * t783 - f64x8::splat(0.0258125) * t89 * t855 * t123 - f64x8::splat(0.0258125) * t210 * t849 * t332 - f64x8::splat(0.051625) * t801 * t802;
            let t866 = f64x8::splat(0.30975) * t770 * t812 - f64x8::splat(0.051625) * t779 * t309 - f64x8::splat(0.30975) * t782 * t817 - f64x8::splat(0.10325) * t292 * t820 + f64x8::splat(0.0258125) * t793 * t119 + f64x8::splat(0.0258125) * t796 * t314 + f64x8::splat(0.051625) * t299 * t317 + f64x8::splat(0.051625) * t801 * t829 + f64x8::splat(0.051625) * t302 * t832 + f64x8::splat(0.2065) * t114 * t835 - f64x8::splat(0.10325) * t114 * t838 + f64x8::splat(6.0) * t447 * t841 - f64x8::splat(6.0) * t197 * t844 + t88 * t864;
            let t868 = -f64x8::splat(0.30975) * t770 * t773 + f64x8::splat(0.051625) * t779 * t293 + f64x8::splat(0.30975) * t782 * t784 - f64x8::splat(0.10325) * t292 * t787 - f64x8::splat(0.0258125) * t793 * t116 - f64x8::splat(0.0258125) * t796 * t303 + f64x8::splat(0.051625) * t299 * t306 - f64x8::splat(0.051625) * t801 * t803 + f64x8::splat(0.051625) * t302 * t806 - f64x8::splat(0.10325) * t114 * t809 + t42 * t866;
            let t872 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t868));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t872;
            acc_v3sigma3 = tv3sigma30;
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
        ip += 8;
    }
}
