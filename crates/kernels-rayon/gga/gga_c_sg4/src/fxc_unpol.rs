//! GGA_C_SG4 fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_sg4.c`
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
pub fn gga_c_sg4_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
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
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(v_rho));
            let t10 = t4 * t6 / t7;
            let t12 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t10;
            let t13 = ((t10).sqrt());
            let t16 = ((t10) * (t10).sqrt());
            let t18 = t1 * t1;
            let t19 = t3 * t3;
            let t20 = t18 * t19;
            let t21 = t7 * t7;
            let t24 = t20 * t5 / t21;
            let t26 = f64x8::splat(3.79785) * t13 + f64x8::splat(0.8969) * t10 + f64x8::splat(0.204775) * t16 + f64x8::splat(0.123235) * t24;
            let t29 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t26;
            let t30 = (simd::ln(t29));
            let t32 = f64x8::splat(0.0621814) * t12 * t30;
            let t33 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t34 = (simd::cbrt(zeta_threshold));
            let t36 = ((t33).select(t34 * zeta_threshold, f64x8::splat(1.0)));
            let t39 = f64x8::splat(M_CBRT2);
            let t43 = (f64x8::splat(2.0) * t36 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t39 - f64x8::splat(2.0));
            let t45 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t10;
            let t50 = f64x8::splat(5.1785) * t13 + f64x8::splat(0.905775) * t10 + f64x8::splat(0.1100325) * t16 + f64x8::splat(0.1241775) * t24;
            let t53 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t50;
            let t54 = (simd::ln(t53));
            let t57 = f64x8::splat(0.0197516734986138) * t43 * t45 * t54;
            let t58 = t34 * t34;
            let t59 = ((t33).select(t58, f64x8::splat(1.0)));
            let t60 = ((v_sigma).sqrt());
            let t61 = t60 * v_sigma;
            let t62 = v_rho * v_rho;
            let t63 = t62 * t62;
            let t64 = f64x8::splat(1.0) / t63;
            let t66 = t59 * t59;
            let t67 = t66 * t59;
            let t68 = f64x8::splat(1.0) / t67;
            let t70 = f64x8::splat(1.0) / t13 / t10;
            let t71 = t68 * t70;
            let t74 = (simd::pow(t59, f64x8::splat(0.05) * t61 * t64 * t71));
            let t75 = (simd::ln(f64x8::splat(2.0)));
            let t76 = f64x8::splat(1.0) - t75;
            let t77 = t74 * t76;
            let t78 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t79 = f64x8::splat(1.0) / t78;
            let t80 = t79 * t67;
            let t82 = f64x8::splat(1.0) / t7 / v_rho;
            let t84 = t39 * t39;
            let t86 = f64x8::splat(1.0) / t59;
            let t87 = f64x8::splat(1.0) / t13;
            let t88 = t86 * t87;
            let t90 = (simd::exp(-t24 / f64x8::splat(4.0)));
            let t91 = f64x8::splat(1.0) - t90;
            let t92 = t88 * t91;
            let t95 = f64x8::splat(0.07963845034287749) + f64x8::splat(0.0175) * t60 * t82 * t84 * t92;
            let t97 = f64x8::splat(1.0) / t7 / t62;
            let t100 = f64x8::splat(1.0) / t66;
            let t102 = f64x8::splat(1.0) / t3;
            let t104 = t100 * t18 * t102 * t5;
            let t107 = f64x8::splat(1.0) / t76;
            let t108 = t95 * t107;
            let t113 = (simd::exp(-(-t32 + t57) * t107 * t78 * t68));
            let t114 = t113 - f64x8::splat(1.0);
            let t115 = f64x8::splat(1.0) / t114;
            let t116 = t78 * t115;
            let t117 = v_sigma * v_sigma;
            let t118 = t116 * t117;
            let t119 = t108 * t118;
            let t121 = f64x8::splat(1.0) / t21 / t63;
            let t122 = t121 * t84;
            let t123 = t66 * t66;
            let t124 = f64x8::splat(1.0) / t123;
            let t126 = f64x8::splat(1.0) / t19;
            let t127 = t1 * t126;
            let t128 = t127 * t6;
            let t129 = t122 * t124 * t128;
            let t132 = v_sigma * t97 * t39 * t104 / f64x8::splat(96.0) + t119 * t129 / f64x8::splat(3072.0);
            let t133 = t95 * t132;
            let t134 = t107 * t78;
            let t135 = t116 * t132;
            let t137 = t108 * t135 + f64x8::splat(1.0);
            let t138 = f64x8::splat(1.0) / t137;
            let t139 = t134 * t138;
            let t141 = t133 * t139 + f64x8::splat(1.0);
            let t142 = (simd::ln(t141));
            let t144 = t77 * t80 * t142;
            let tzk0 = -t32 + t57 + t144;
            acc_zk = tzk0;
            let t145 = t6 * t82;
            let t147 = t4 * t145 * t30;
            let t148 = f64x8::splat(0.0011073470983333333) * t147;
            let t149 = t26 * t26;
            let t150 = f64x8::splat(1.0) / t149;
            let t151 = t12 * t150;
            let t152 = t87 * t1;
            let t153 = t3 * t6;
            let t154 = t153 * t82;
            let t155 = t152 * t154;
            let t157 = t4 * t145;
            let t159 = ((t10).sqrt());
            let t160 = t159 * t1;
            let t161 = t160 * t154;
            let t165 = t5 / t21 / v_rho;
            let t166 = t20 * t165;
            let t168 = -f64x8::splat(0.632975) * t155 - f64x8::splat(0.29896666666666666) * t157 - f64x8::splat(0.1023875) * t161 - f64x8::splat(0.08215666666666667) * t166;
            let t169 = f64x8::splat(1.0) / t29;
            let t170 = t168 * t169;
            let t171 = t151 * t170;
            let t172 = f64x8::splat(1.0) * t171;
            let t173 = t43 * t1;
            let t176 = t173 * t153 * t82 * t54;
            let t177 = f64x8::splat(0.00018311447306006544) * t176;
            let t178 = t43 * t45;
            let t179 = t50 * t50;
            let t180 = f64x8::splat(1.0) / t179;
            let t185 = -f64x8::splat(0.8630833333333333) * t155 - f64x8::splat(0.301925) * t157 - f64x8::splat(0.05501625) * t161 - f64x8::splat(0.082785) * t166;
            let t187 = f64x8::splat(1.0) / t53;
            let t188 = t180 * t185 * t187;
            let t189 = t178 * t188;
            let t190 = f64x8::splat(0.5848223622634646) * t189;
            let t191 = t63 * v_rho;
            let t192 = f64x8::splat(1.0) / t191;
            let t197 = f64x8::splat(1.0) / t7 / t191;
            let t202 = f64x8::splat(1.0) / t13 / t24 / f64x8::splat(4.0);
            let t203 = t202 * t1;
            let t204 = t203 * t153;
            let t207 = -f64x8::splat(0.2) * t61 * t192 * t71 + f64x8::splat(0.025) * t61 * t197 * t68 * t204;
            let t208 = t74 * t207;
            let t209 = (simd::ln(t59));
            let t211 = t76 * t79;
            let t213 = t211 * t67 * t142;
            let t214 = t208 * t209 * t213;
            let t215 = t77 * t79;
            let t221 = f64x8::splat(1.0) / t21 / t62;
            let t223 = t84 * t86;
            let t226 = t4 * t6;
            let t227 = t70 * t91 * t226;
            let t230 = t62 * v_rho;
            let t231 = f64x8::splat(1.0) / t230;
            let t235 = t19 * t5;
            let t236 = t235 * t90;
            let t237 = t87 * t18 * t236;
            let t240 = -f64x8::splat(0.023333333333333334) * t60 * t97 * t84 * t92 + f64x8::splat(0.002916666666666667) * t60 * t221 * t223 * t227 - f64x8::splat(0.002916666666666667) * t60 * t231 * t223 * t237;
            let t241 = t240 * t132;
            let t244 = f64x8::splat(1.0) / t7 / t230;
            let t249 = t240 * t107;
            let t250 = t249 * t118;
            let t253 = t76 * t76;
            let t254 = f64x8::splat(1.0) / t253;
            let t255 = t95 * t254;
            let t256 = t78 * t78;
            let t257 = t255 * t256;
            let t258 = t114 * t114;
            let t259 = f64x8::splat(1.0) / t258;
            let t260 = t259 * t117;
            let t261 = t260 * t121;
            let t262 = t257 * t261;
            let t264 = f64x8::splat(1.0) / t123 / t67;
            let t266 = t84 * t264 * t1;
            let t267 = t126 * t6;
            let t268 = t148 + t172 - t177 - t190;
            let t269 = t268 * t113;
            let t271 = t266 * t267 * t269;
            let t275 = f64x8::splat(1.0) / t21 / t191;
            let t276 = t275 * t84;
            let t278 = t276 * t124 * t128;
            let t281 = -f64x8::splat(7.0) / f64x8::splat(288.0) * v_sigma * t244 * t39 * t104 + t250 * t129 / f64x8::splat(3072.0) + t262 * t271 / f64x8::splat(3072.0) - f64x8::splat(7.0) / f64x8::splat(4608.0) * t119 * t278;
            let t282 = t95 * t281;
            let t284 = t133 * t107;
            let t285 = t137 * t137;
            let t286 = f64x8::splat(1.0) / t285;
            let t287 = t78 * t286;
            let t289 = t256 * t259;
            let t290 = t255 * t289;
            let t291 = t132 * t268;
            let t292 = t68 * t113;
            let t293 = t291 * t292;
            let t295 = t116 * t281;
            let t297 = t108 * t295 + t249 * t135 + t290 * t293;
            let t298 = t287 * t297;
            let t300 = t241 * t139 + t282 * t139 - t284 * t298;
            let t302 = f64x8::splat(1.0) / t141;
            let t303 = t67 * t300 * t302;
            let t304 = t215 * t303;
            let tvrho0 = -t32 + t57 + t144 + v_rho * (t148 + t172 - t177 - t190 + t214 + t304);
            acc_vrho = tvrho0;
            let t307 = t74 * t60;
            let t310 = t76 * t142;
            let t311 = t70 * t209 * t310;
            let t313 = f64x8::splat(0.007599088773175333) * t307 * t64 * t311;
            let t314 = f64x8::splat(1.0) / t60;
            let t315 = t314 * t82;
            let t316 = t315 * t223;
            let t317 = t87 * t91;
            let t318 = t132 * t107;
            let t319 = t318 * t138;
            let t320 = t317 * t319;
            let t325 = t18 * t102;
            let t326 = t325 * t5;
            let t329 = t63 * t62;
            let t330 = f64x8::splat(1.0) / t329;
            let t331 = t61 * t330;
            let t332 = t123 * t59;
            let t333 = f64x8::splat(1.0) / t332;
            let t334 = t39 * t333;
            let t335 = t334 * t87;
            let t337 = t91 * t107;
            let t338 = t337 * t115;
            let t339 = t338 * t128;
            let t342 = t116 * v_sigma;
            let t343 = t108 * t342;
            let t346 = t97 * t39 * t100 * t326 / f64x8::splat(96.0) + f64x8::splat(5.622333236297649e-05) * t331 * t335 * t339 + t343 * t129 / f64x8::splat(1536.0);
            let t347 = t95 * t346;
            let t349 = t107 * t115;
            let t350 = t349 * t132;
            let t351 = t317 * t350;
            let t354 = t116 * t346;
            let t356 = f64x8::splat(0.08635903850953189) * t316 * t351 + t108 * t354;
            let t357 = t287 * t356;
            let t359 = f64x8::splat(0.08635903850953189) * t316 * t320 + t347 * t139 - t284 * t357;
            let t360 = t67 * t359;
            let t361 = t360 * t302;
            let t362 = t215 * t361;
            let tvsigma0 = v_rho * (t313 + t362);
            acc_vsigma = tvsigma0;
            let t370 = t6 * t97;
            let t372 = t4 * t370 * t30;
            let t373 = f64x8::splat(0.0014764627977777779) * t372;
            let t374 = t82 * t150;
            let t376 = t226 * t374 * t170;
            let t377 = f64x8::splat(0.035616666666666665) * t376;
            let t378 = t149 * t26;
            let t379 = f64x8::splat(1.0) / t378;
            let t380 = t12 * t379;
            let t381 = t168 * t168;
            let t382 = t381 * t169;
            let t383 = t380 * t382;
            let t384 = f64x8::splat(2.0) * t383;
            let t385 = t70 * t18;
            let t386 = t235 * t221;
            let t387 = t385 * t386;
            let t389 = t153 * t97;
            let t390 = t152 * t389;
            let t392 = t4 * t370;
            let t394 = f64x8::splat(1.0)/((t10).sqrt());
            let t395 = t394 * t18;
            let t396 = t395 * t386;
            let t398 = t160 * t389;
            let t401 = t20 * t5 * t221;
            let t403 = -f64x8::splat(0.4219833333333333) * t387 + f64x8::splat(0.8439666666666666) * t390 + f64x8::splat(0.3986222222222222) * t392 + f64x8::splat(0.06825833333333334) * t396 + f64x8::splat(0.13651666666666668) * t398 + f64x8::splat(0.1369277777777778) * t401;
            let t404 = t403 * t169;
            let t405 = t151 * t404;
            let t406 = f64x8::splat(1.0) * t405;
            let t407 = t149 * t149;
            let t408 = f64x8::splat(1.0) / t407;
            let t409 = t12 * t408;
            let t410 = t29 * t29;
            let t411 = f64x8::splat(1.0) / t410;
            let t412 = t381 * t411;
            let t413 = t409 * t412;
            let t414 = f64x8::splat(16.081979498692537) * t413;
            let t417 = t173 * t153 * t97 * t54;
            let t418 = f64x8::splat(0.00024415263074675396) * t417;
            let t419 = t43 * t4;
            let t421 = t419 * t145 * t188;
            let t422 = f64x8::splat(0.01084358130030174) * t421;
            let t423 = t179 * t50;
            let t424 = f64x8::splat(1.0) / t423;
            let t425 = t185 * t185;
            let t427 = t424 * t425 * t187;
            let t428 = t178 * t427;
            let t429 = f64x8::splat(1.1696447245269292) * t428;
            let t436 = -f64x8::splat(0.5753888888888888) * t387 + f64x8::splat(1.1507777777777777) * t390 + f64x8::splat(0.4025666666666667) * t392 + f64x8::splat(0.0366775) * t396 + f64x8::splat(0.073355) * t398 + f64x8::splat(0.137975) * t401;
            let t438 = t180 * t436 * t187;
            let t439 = t178 * t438;
            let t440 = f64x8::splat(0.5848223622634646) * t439;
            let t441 = t179 * t179;
            let t442 = f64x8::splat(1.0) / t441;
            let t443 = t442 * t425;
            let t444 = t53 * t53;
            let t445 = f64x8::splat(1.0) / t444;
            let t446 = t443 * t445;
            let t447 = t178 * t446;
            let t448 = f64x8::splat(17.315859105681465) * t447;
            let t449 = t207 * t207;
            let t450 = t74 * t449;
            let t451 = t209 * t209;
            let t453 = t450 * t451 * t213;
            let t457 = f64x8::splat(1.0) / t7 / t329;
            let t463 = f64x8::splat(1.0) / t21 / t329;
            let t470 = f64x8::splat(1.0) / t13 / t2 * v_rho / f64x8::splat(48.0);
            let t471 = t470 * t18;
            let t472 = t471 * t235;
            let t475 = f64x8::splat(1.0) * t331 * t71 - f64x8::splat(0.23333333333333334) * t61 * t457 * t68 * t204 + f64x8::splat(0.08333333333333333) * t61 * t463 * t68 * t472;
            let t476 = t74 * t475;
            let t478 = t476 * t209 * t213;
            let t479 = t209 * t76;
            let t480 = t208 * t479;
            let t481 = t300 * t302;
            let t482 = t80 * t481;
            let t483 = t480 * t482;
            let t490 = f64x8::splat(1.0) / t21 / t230;
            let t495 = t60 * t64;
            let t496 = t495 * t223;
            let t499 = t202 * t91;
            let t500 = t20 * t5;
            let t501 = t499 * t500;
            let t505 = f64x8::splat(1.0) / t7 / t63;
            let t507 = t60 * t505 * t84;
            let t508 = t86 * t70;
            let t509 = t2 * t90;
            let t510 = t508 * t509;
            let t514 = t60 * t121 * t223;
            let t515 = t3 * t2;
            let t517 = t515 * t6 * t90;
            let t518 = t152 * t517;
            let t521 = f64x8::splat(0.05444444444444444) * t60 * t244 * t84 * t92 - f64x8::splat(0.011666666666666667) * t60 * t490 * t223 * t227 + f64x8::splat(0.012638888888888889) * t496 * t237 + f64x8::splat(0.005833333333333334) * t496 * t501 - f64x8::splat(0.011666666666666667) * t507 * t510 - f64x8::splat(0.0014583333333333334) * t514 * t518;
            let t522 = t521 * t132;
            let t524 = t240 * t281;
            let t527 = t241 * t107;
            let t534 = t521 * t107;
            let t535 = t534 * t118;
            let t538 = t240 * t254;
            let t539 = t538 * t256;
            let t540 = t539 * t261;
            let t546 = f64x8::splat(1.0) / t253 / t76;
            let t547 = t95 * t546;
            let t548 = t256 * t78;
            let t549 = t547 * t548;
            let t551 = f64x8::splat(1.0) / t258 / t114;
            let t552 = t551 * t117;
            let t553 = t552 * t121;
            let t554 = t549 * t553;
            let t555 = t123 * t123;
            let t557 = f64x8::splat(1.0) / t555 / t66;
            let t559 = t84 * t557 * t1;
            let t560 = t268 * t268;
            let t561 = t113 * t113;
            let t564 = t559 * t267 * t560 * t561;
            let t567 = t260 * t275;
            let t568 = t257 * t567;
            let t571 = -t373 - t377 - t384 + t406 + t414 + t418 + t422 + t429 - t440 - t448;
            let t572 = t571 * t113;
            let t574 = t266 * t267 * t572;
            let t577 = t549 * t261;
            let t580 = t559 * t267 * t560 * t113;
            let t585 = t463 * t84 * t124 * t128;
            let t588 = f64x8::splat(35.0) / f64x8::splat(432.0) * v_sigma * t505 * t39 * t104 + t535 * t129 / f64x8::splat(3072.0) + t540 * t271 / f64x8::splat(1536.0) - f64x8::splat(7.0) / f64x8::splat(2304.0) * t250 * t278 + t554 * t564 / f64x8::splat(1536.0) - f64x8::splat(7.0) / f64x8::splat(2304.0) * t568 * t271 + t262 * t574 / f64x8::splat(3072.0) - t577 * t580 / f64x8::splat(3072.0) + f64x8::splat(119.0) / f64x8::splat(13824.0) * t119 * t585;
            let t589 = t95 * t588;
            let t591 = t282 * t107;
            let t595 = f64x8::splat(1.0) / t285 / t137;
            let t596 = t78 * t595;
            let t597 = t297 * t297;
            let t598 = t596 * t597;
            let t602 = t538 * t289;
            let t607 = t548 * t551;
            let t608 = t547 * t607;
            let t609 = t132 * t560;
            let t610 = t123 * t66;
            let t611 = f64x8::splat(1.0) / t610;
            let t612 = t611 * t561;
            let t613 = t609 * t612;
            let t616 = t281 * t268;
            let t617 = t616 * t292;
            let t620 = t132 * t571;
            let t621 = t620 * t292;
            let t623 = t548 * t259;
            let t624 = t547 * t623;
            let t625 = t611 * t113;
            let t626 = t609 * t625;
            let t628 = t116 * t588;
            let t630 = t108 * t628 + t534 * t135 + f64x8::splat(2.0) * t249 * t295 + f64x8::splat(2.0) * t290 * t617 + t290 * t621 + f64x8::splat(2.0) * t602 * t293 + f64x8::splat(2.0) * t608 * t613 - t624 * t626;
            let t631 = t287 * t630;
            let t633 = t522 * t139 + f64x8::splat(2.0) * t524 * t139 + t589 * t139 + f64x8::splat(2.0) * t284 * t598 - t284 * t631 - f64x8::splat(2.0) * t527 * t298 - f64x8::splat(2.0) * t591 * t298;
            let t634 = t67 * t633;
            let t636 = t215 * t634 * t302;
            let t637 = t300 * t300;
            let t638 = t67 * t637;
            let t639 = t141 * t141;
            let t640 = f64x8::splat(1.0) / t639;
            let t642 = t215 * t638 * t640;
            let t643 = -t373 - t377 - t384 + t406 + t414 + t418 + t422 + t429 - t440 - t448 + t453 + t478 + f64x8::splat(2.0) * t483 + t636 - t642;
            let tv2rho20 = f64x8::splat(0.0022146941966666666) * t147 + f64x8::splat(2.0) * t171 - f64x8::splat(0.0003662289461201309) * t176 - f64x8::splat(1.1696447245269292) * t189 + f64x8::splat(2.0) * t214 + f64x8::splat(2.0) * t304 + v_rho * t643;
            acc_v2rho2 = tv2rho20;
            let t645 = t451 * t60;
            let t646 = t208 * t645;
            let t647 = t64 * t70;
            let t648 = t647 * t310;
            let t649 = t646 * t648;
            let t652 = t307 * t192 * t311;
            let t655 = t197 * t202 * t209;
            let t656 = t307 * t655;
            let t657 = t310 * t226;
            let t658 = t656 * t657;
            let t660 = t307 * t647;
            let t661 = t479 * t481;
            let t662 = t660 * t661;
            let t664 = t359 * t302;
            let t665 = t80 * t664;
            let t666 = t480 * t665;
            let t667 = t314 * t97;
            let t668 = t667 * t223;
            let t672 = t314 * t221 * t84;
            let t673 = t508 * t91;
            let t674 = t672 * t673;
            let t675 = t319 * t226;
            let t679 = t314 * t231 * t84;
            let t680 = t88 * t18;
            let t681 = t679 * t680;
            let t682 = t236 * t319;
            let t685 = t281 * t107;
            let t686 = t685 * t138;
            let t687 = t317 * t686;
            let t690 = t223 * t87;
            let t691 = t315 * t690;
            let t692 = t91 * t132;
            let t693 = t107 * t286;
            let t694 = t693 * t297;
            let t695 = t692 * t694;
            let t698 = t240 * t346;
            let t704 = t63 * t230;
            let t705 = f64x8::splat(1.0) / t704;
            let t706 = t61 * t705;
            let t711 = f64x8::splat(1.0) / t7 / t704;
            let t712 = t61 * t711;
            let t713 = t334 * t70;
            let t715 = t338 * t326;
            let t719 = f64x8::splat(1.0) / t21 / t704;
            let t720 = t61 * t719;
            let t722 = t87 * t90;
            let t723 = t722 * t349;
            let t726 = t331 * t39;
            let t727 = f64x8::splat(1.0) / t555;
            let t728 = t727 * t87;
            let t729 = t91 * t254;
            let t730 = t728 * t729;
            let t731 = t726 * t730;
            let t732 = t259 * t1;
            let t733 = t732 * t126;
            let t734 = t6 * t268;
            let t735 = t78 * t113;
            let t736 = t734 * t735;
            let t737 = t733 * t736;
            let t740 = t249 * t342;
            let t743 = t259 * v_sigma;
            let t744 = t743 * t121;
            let t745 = t257 * t744;
            let t750 = -f64x8::splat(7.0) / f64x8::splat(288.0) * t244 * t39 * t100 * t326 - f64x8::splat(0.00033733999417785894) * t706 * t335 * t339 + f64x8::splat(3.748222157531766e-05) * t712 * t713 * t715 - f64x8::splat(0.00011244666472595298) * t720 * t334 * t723 + f64x8::splat(5.622333236297649e-05) * t731 * t737 + t740 * t129 / f64x8::splat(1536.0) + t745 * t271 / f64x8::splat(1536.0) - f64x8::splat(7.0) / f64x8::splat(2304.0) * t343 * t278;
            let t751 = t95 * t750;
            let t753 = t347 * t107;
            let t757 = t356 * t297;
            let t758 = t596 * t757;
            let t763 = t350 * t226;
            let t766 = t236 * t350;
            let t769 = t315 * t84;
            let t770 = t124 * t87;
            let t771 = t770 * t91;
            let t772 = t769 * t771;
            let t773 = t254 * t259;
            let t774 = t773 * t132;
            let t775 = t268 * t78;
            let t776 = t775 * t113;
            let t777 = t774 * t776;
            let t780 = t349 * t281;
            let t781 = t317 * t780;
            let t785 = t346 * t268;
            let t786 = t785 * t292;
            let t788 = t116 * t750;
            let t790 = -f64x8::splat(0.11514538467937585) * t668 * t351 + f64x8::splat(0.014393173084921981) * t674 * t763 - f64x8::splat(0.014393173084921981) * t681 * t766 + f64x8::splat(0.08635903850953189) * t772 * t777 + f64x8::splat(0.08635903850953189) * t316 * t781 + t249 * t354 + t290 * t786 + t108 * t788;
            let t791 = t287 * t790;
            let t793 = -f64x8::splat(0.11514538467937585) * t668 * t320 + f64x8::splat(0.014393173084921981) * t674 * t675 - f64x8::splat(0.014393173084921981) * t681 * t682 + f64x8::splat(0.08635903850953189) * t316 * t687 - f64x8::splat(0.08635903850953189) * t691 * t695 + t698 * t139 + t751 * t139 - t753 * t298 - t527 * t357 - t591 * t357 + f64x8::splat(2.0) * t284 * t758 - t284 * t791;
            let t794 = t67 * t793;
            let t796 = t215 * t794 * t302;
            let t797 = t640 * t300;
            let t799 = t215 * t360 * t797;
            let tv2rhosigma0 = t313 + t362 + v_rho * (f64x8::splat(0.007599088773175333) * t649 - f64x8::splat(0.030396355092701333) * t652 + f64x8::splat(0.0037995443865876666) * t658 + f64x8::splat(0.007599088773175333) * t662 + t666 + t796 - t799);
            acc_v2rhosigma = tv2rhosigma0;
            let t802 = t74 * v_sigma;
            let t805 = t68 * t451 * t310;
            let t807 = f64x8::splat(3.730193978716297e-05) * t802 * t705 * t805;
            let t808 = t74 * t314;
            let t811 = f64x8::splat(0.0037995443865876666) * t808 * t64 * t311;
            let t812 = t479 * t664;
            let t814 = f64x8::splat(0.015198177546350666) * t660 * t812;
            let t815 = f64x8::splat(1.0) / t61;
            let t816 = t815 * t82;
            let t817 = t816 * t223;
            let t820 = t346 * t107;
            let t821 = t820 * t138;
            let t822 = t317 * t821;
            let t825 = t693 * t356;
            let t826 = t692 * t825;
            let t829 = t60 * t330;
            let t833 = t116 * t121;
            let t836 = t84 * t124 * t128;
            let t839 = f64x8::splat(0.0001967816632704177) * t829 * t335 * t339 + t108 * t833 * t836 / f64x8::splat(1536.0);
            let t840 = t95 * t839;
            let t844 = t356 * t356;
            let t845 = t596 * t844;
            let t850 = t349 * t346;
            let t851 = t317 * t850;
            let t854 = t116 * t839;
            let t856 = -f64x8::splat(0.043179519254765944) * t817 * t351 + f64x8::splat(0.17271807701906378) * t316 * t851 + t108 * t854;
            let t857 = t287 * t856;
            let t859 = -f64x8::splat(0.043179519254765944) * t817 * t320 + f64x8::splat(0.17271807701906378) * t316 * t822 - f64x8::splat(0.17271807701906378) * t691 * t826 + t840 * t139 - f64x8::splat(2.0) * t753 * t357 + f64x8::splat(2.0) * t284 * t845 - t284 * t857;
            let t860 = t67 * t859;
            let t862 = t215 * t860 * t302;
            let t863 = t359 * t359;
            let t864 = t67 * t863;
            let t866 = t215 * t864 * t640;
            let tv2sigma20 = v_rho * (t807 + t811 + t814 + t862 - t866);
            acc_v2sigma2 = tv2sigma20;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        ip += 8;
    }
}
