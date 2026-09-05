//! GGA_C_PBELOC fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_pbeloc.c`
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
pub fn gga_c_pbeloc_fxc_unpol(
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
            let t58 = (simd::ln(f64x8::splat(2.0)));
            let t59 = f64x8::splat(1.0) - t58;
            let t60 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t61 = f64x8::splat(1.0) / t60;
            let t62 = t59 * t61;
            let t63 = t34 * t34;
            let t64 = ((t33).select(t63, f64x8::splat(1.0)));
            let t65 = t64 * t64;
            let t66 = t65 * t64;
            let t67 = v_rho * v_rho;
            let t69 = f64x8::splat(1.0) / t7 / t67;
            let t70 = v_sigma * t69;
            let t71 = f64x8::splat(1.0) / t65;
            let t72 = t39 * t71;
            let t74 = f64x8::splat(1.0) / t3;
            let t75 = t18 * t74;
            let t77 = (simd::exp(-t24 / f64x8::splat(4.0)));
            let t78 = f64x8::splat(1.0) - t77;
            let t79 = t5 * t78;
            let t80 = t75 * t79;
            let t83 = f64x8::splat(0.0375) + f64x8::splat(0.0008333333333333334) * t70 * t72 * t80;
            let t85 = t71 * t18;
            let t87 = t85 * t74 * t5;
            let t90 = f64x8::splat(1.0) / t59;
            let t91 = t83 * t90;
            let t94 = f64x8::splat(1.0) / t66;
            let t97 = (simd::exp(-(-t32 + t57) * t90 * t60 * t94));
            let t98 = t97 - f64x8::splat(1.0);
            let t99 = f64x8::splat(1.0) / t98;
            let t100 = t60 * t99;
            let t101 = v_sigma * v_sigma;
            let t102 = t100 * t101;
            let t103 = t91 * t102;
            let t104 = t67 * t67;
            let t106 = f64x8::splat(1.0) / t21 / t104;
            let t107 = t39 * t39;
            let t108 = t106 * t107;
            let t109 = t65 * t65;
            let t110 = f64x8::splat(1.0) / t109;
            let t112 = f64x8::splat(1.0) / t19;
            let t114 = t1 * t112 * t6;
            let t115 = t108 * t110 * t114;
            let t118 = t70 * t39 * t87 / f64x8::splat(96.0) + t103 * t115 / f64x8::splat(3072.0);
            let t119 = t83 * t118;
            let t120 = t90 * t60;
            let t121 = t100 * t118;
            let t123 = t121 * t91 + f64x8::splat(1.0);
            let t124 = f64x8::splat(1.0) / t123;
            let t125 = t120 * t124;
            let t127 = t119 * t125 + f64x8::splat(1.0);
            let t128 = (simd::ln(t127));
            let t130 = t62 * t66 * t128;
            let tzk0 = -t32 + t57 + t130;
            acc_zk = tzk0;
            let t132 = f64x8::splat(1.0) / t7 / v_rho;
            let t133 = t6 * t132;
            let t135 = t4 * t133 * t30;
            let t136 = f64x8::splat(0.0011073470983333333) * t135;
            let t137 = t26 * t26;
            let t138 = f64x8::splat(1.0) / t137;
            let t139 = t12 * t138;
            let t141 = f64x8::splat(1.0) / t13 * t1;
            let t142 = t3 * t6;
            let t143 = t142 * t132;
            let t144 = t141 * t143;
            let t146 = t4 * t133;
            let t148 = ((t10).sqrt());
            let t149 = t148 * t1;
            let t150 = t149 * t143;
            let t155 = t20 * t5 / t21 / v_rho;
            let t157 = -f64x8::splat(0.632975) * t144 - f64x8::splat(0.29896666666666666) * t146 - f64x8::splat(0.1023875) * t150 - f64x8::splat(0.08215666666666667) * t155;
            let t158 = f64x8::splat(1.0) / t29;
            let t159 = t157 * t158;
            let t160 = t139 * t159;
            let t161 = f64x8::splat(1.0) * t160;
            let t162 = t43 * t1;
            let t165 = t162 * t142 * t132 * t54;
            let t166 = f64x8::splat(0.00018311447306006544) * t165;
            let t167 = t43 * t45;
            let t168 = t50 * t50;
            let t169 = f64x8::splat(1.0) / t168;
            let t174 = -f64x8::splat(0.8630833333333333) * t144 - f64x8::splat(0.301925) * t146 - f64x8::splat(0.05501625) * t150 - f64x8::splat(0.082785) * t155;
            let t176 = f64x8::splat(1.0) / t53;
            let t177 = t169 * t174 * t176;
            let t178 = t167 * t177;
            let t179 = f64x8::splat(0.5848223622634646) * t178;
            let t180 = t67 * v_rho;
            let t182 = f64x8::splat(1.0) / t7 / t180;
            let t183 = v_sigma * t182;
            let t187 = f64x8::splat(1.0) / t104;
            let t190 = t6 * t77;
            let t191 = t4 * t190;
            let t194 = -f64x8::splat(0.0019444444444444444) * t183 * t72 * t80 - f64x8::splat(0.0004166666666666667) * v_sigma * t187 * t72 * t191;
            let t195 = t194 * t118;
            let t200 = t194 * t90;
            let t201 = t200 * t102;
            let t204 = t59 * t59;
            let t205 = f64x8::splat(1.0) / t204;
            let t206 = t83 * t205;
            let t207 = t60 * t60;
            let t208 = t206 * t207;
            let t209 = t98 * t98;
            let t210 = f64x8::splat(1.0) / t209;
            let t211 = t210 * t101;
            let t212 = t211 * t106;
            let t213 = t208 * t212;
            let t214 = t109 * t66;
            let t215 = f64x8::splat(1.0) / t214;
            let t217 = t107 * t215 * t1;
            let t218 = t112 * t6;
            let t219 = t136 + t161 - t166 - t179;
            let t220 = t219 * t97;
            let t222 = t217 * t218 * t220;
            let t225 = t104 * v_rho;
            let t227 = f64x8::splat(1.0) / t21 / t225;
            let t228 = t227 * t107;
            let t230 = t228 * t110 * t114;
            let t233 = -f64x8::splat(7.0) / f64x8::splat(288.0) * t183 * t39 * t87 + t201 * t115 / f64x8::splat(3072.0) + t213 * t222 / f64x8::splat(3072.0) - f64x8::splat(7.0) / f64x8::splat(4608.0) * t103 * t230;
            let t234 = t83 * t233;
            let t236 = t119 * t90;
            let t237 = t123 * t123;
            let t238 = f64x8::splat(1.0) / t237;
            let t239 = t60 * t238;
            let t241 = t207 * t210;
            let t242 = t206 * t241;
            let t243 = t118 * t219;
            let t244 = t94 * t97;
            let t245 = t243 * t244;
            let t247 = t100 * t233;
            let t249 = t121 * t200 + t242 * t245 + t247 * t91;
            let t250 = t239 * t249;
            let t252 = t125 * t195 + t125 * t234 - t236 * t250;
            let t254 = f64x8::splat(1.0) / t127;
            let t256 = t62 * t66 * t252 * t254;
            let tvrho0 = -t32 + t57 + t130 + v_rho * (t136 + t161 - t166 - t179 + t256);
            acc_vrho = tvrho0;
            let t259 = v_rho * t59;
            let t260 = t259 * t61;
            let t261 = t69 * t39;
            let t262 = t85 * t74;
            let t263 = t261 * t262;
            let t265 = t118 * t90 * t124;
            let t266 = t79 * t265;
            let t270 = t75 * t5;
            let t271 = t261 * t71 * t270;
            let t273 = t104 * t180;
            let t274 = f64x8::splat(1.0) / t273;
            let t276 = f64x8::splat(1.0) / t109 / t65;
            let t278 = t274 * t276 * t78;
            let t279 = t90 * t99;
            let t280 = t279 * t101;
            let t283 = t100 * v_sigma;
            let t284 = t91 * t283;
            let t287 = t271 / f64x8::splat(96.0) + f64x8::splat(0.00020186378047070194) * t278 * t280 + t284 * t115 / f64x8::splat(1536.0);
            let t288 = t83 * t287;
            let t290 = t279 * t118;
            let t291 = t79 * t290;
            let t294 = t100 * t287;
            let t296 = f64x8::splat(0.008224670334241133) * t263 * t291 + t91 * t294;
            let t297 = t239 * t296;
            let t299 = f64x8::splat(0.008224670334241133) * t263 * t266 + t288 * t125 - t236 * t297;
            let t300 = t66 * t299;
            let t301 = t300 * t254;
            let tvsigma0 = t260 * t301;
            acc_vsigma = tvsigma0;
            let t307 = t6 * t69;
            let t309 = t4 * t307 * t30;
            let t310 = f64x8::splat(0.0014764627977777779) * t309;
            let t311 = t4 * t6;
            let t312 = t132 * t138;
            let t314 = t311 * t312 * t159;
            let t315 = f64x8::splat(0.035616666666666665) * t314;
            let t316 = t137 * t26;
            let t317 = f64x8::splat(1.0) / t316;
            let t318 = t12 * t317;
            let t319 = t157 * t157;
            let t320 = t319 * t158;
            let t321 = t318 * t320;
            let t322 = f64x8::splat(2.0) * t321;
            let t325 = f64x8::splat(1.0) / t13 / t10 * t18;
            let t326 = t19 * t5;
            let t328 = f64x8::splat(1.0) / t21 / t67;
            let t329 = t326 * t328;
            let t330 = t325 * t329;
            let t332 = t142 * t69;
            let t333 = t141 * t332;
            let t335 = t4 * t307;
            let t337 = f64x8::splat(1.0)/((t10).sqrt());
            let t338 = t337 * t18;
            let t339 = t338 * t329;
            let t341 = t149 * t332;
            let t344 = t20 * t5 * t328;
            let t346 = -f64x8::splat(0.4219833333333333) * t330 + f64x8::splat(0.8439666666666666) * t333 + f64x8::splat(0.3986222222222222) * t335 + f64x8::splat(0.06825833333333334) * t339 + f64x8::splat(0.13651666666666668) * t341 + f64x8::splat(0.1369277777777778) * t344;
            let t347 = t346 * t158;
            let t348 = t139 * t347;
            let t349 = f64x8::splat(1.0) * t348;
            let t350 = t137 * t137;
            let t351 = f64x8::splat(1.0) / t350;
            let t352 = t12 * t351;
            let t353 = t29 * t29;
            let t354 = f64x8::splat(1.0) / t353;
            let t355 = t319 * t354;
            let t356 = t352 * t355;
            let t357 = f64x8::splat(16.081979498692537) * t356;
            let t360 = t162 * t142 * t69 * t54;
            let t361 = f64x8::splat(0.00024415263074675396) * t360;
            let t362 = t43 * t4;
            let t364 = t362 * t133 * t177;
            let t365 = f64x8::splat(0.01084358130030174) * t364;
            let t366 = t168 * t50;
            let t367 = f64x8::splat(1.0) / t366;
            let t368 = t174 * t174;
            let t370 = t367 * t368 * t176;
            let t371 = t167 * t370;
            let t372 = f64x8::splat(1.1696447245269292) * t371;
            let t379 = -f64x8::splat(0.5753888888888888) * t330 + f64x8::splat(1.1507777777777777) * t333 + f64x8::splat(0.4025666666666667) * t335 + f64x8::splat(0.0366775) * t339 + f64x8::splat(0.073355) * t341 + f64x8::splat(0.137975) * t344;
            let t381 = t169 * t379 * t176;
            let t382 = t167 * t381;
            let t383 = f64x8::splat(0.5848223622634646) * t382;
            let t384 = t168 * t168;
            let t385 = f64x8::splat(1.0) / t384;
            let t386 = t385 * t368;
            let t387 = t53 * t53;
            let t388 = f64x8::splat(1.0) / t387;
            let t389 = t386 * t388;
            let t390 = t167 * t389;
            let t391 = f64x8::splat(17.315859105681465) * t390;
            let t393 = f64x8::splat(1.0) / t7 / t104;
            let t394 = v_sigma * t393;
            let t398 = f64x8::splat(1.0) / t225;
            let t405 = t71 * t2;
            let t406 = t405 * t77;
            let t409 = f64x8::splat(0.006481481481481481) * t394 * t72 * t80 + f64x8::splat(0.002638888888888889) * v_sigma * t398 * t72 * t191 - f64x8::splat(0.0008333333333333334) * v_sigma * t227 * t39 * t406;
            let t410 = t409 * t118;
            let t412 = t194 * t233;
            let t415 = t195 * t90;
            let t421 = t409 * t90;
            let t422 = t421 * t102;
            let t425 = t194 * t205;
            let t426 = t425 * t207;
            let t427 = t426 * t212;
            let t433 = f64x8::splat(1.0) / t204 / t59;
            let t434 = t83 * t433;
            let t435 = t207 * t60;
            let t436 = t434 * t435;
            let t438 = f64x8::splat(1.0) / t209 / t98;
            let t439 = t438 * t101;
            let t440 = t439 * t106;
            let t441 = t436 * t440;
            let t442 = t109 * t109;
            let t444 = f64x8::splat(1.0) / t442 / t65;
            let t446 = t107 * t444 * t1;
            let t447 = t219 * t219;
            let t448 = t97 * t97;
            let t451 = t446 * t218 * t447 * t448;
            let t454 = t211 * t227;
            let t455 = t208 * t454;
            let t458 = -t310 - t315 - t322 + t349 + t357 + t361 + t365 + t372 - t383 - t391;
            let t459 = t458 * t97;
            let t461 = t217 * t218 * t459;
            let t464 = t436 * t212;
            let t467 = t446 * t218 * t447 * t97;
            let t470 = t104 * t67;
            let t472 = f64x8::splat(1.0) / t21 / t470;
            let t475 = t472 * t107 * t110 * t114;
            let t478 = f64x8::splat(35.0) / f64x8::splat(432.0) * t394 * t39 * t87 + t422 * t115 / f64x8::splat(3072.0) + t427 * t222 / f64x8::splat(1536.0) - f64x8::splat(7.0) / f64x8::splat(2304.0) * t201 * t230 + t441 * t451 / f64x8::splat(1536.0) - f64x8::splat(7.0) / f64x8::splat(2304.0) * t455 * t222 + t213 * t461 / f64x8::splat(3072.0) - t464 * t467 / f64x8::splat(3072.0) + f64x8::splat(119.0) / f64x8::splat(13824.0) * t103 * t475;
            let t479 = t83 * t478;
            let t481 = t234 * t90;
            let t485 = f64x8::splat(1.0) / t237 / t123;
            let t486 = t60 * t485;
            let t487 = t249 * t249;
            let t488 = t486 * t487;
            let t492 = t425 * t241;
            let t497 = t435 * t438;
            let t498 = t434 * t497;
            let t499 = t118 * t447;
            let t500 = t276 * t448;
            let t501 = t499 * t500;
            let t504 = t233 * t219;
            let t505 = t504 * t244;
            let t508 = t118 * t458;
            let t509 = t508 * t244;
            let t511 = t435 * t210;
            let t512 = t434 * t511;
            let t513 = t276 * t97;
            let t514 = t499 * t513;
            let t516 = t100 * t478;
            let t518 = t121 * t421 + f64x8::splat(2.0) * t200 * t247 + f64x8::splat(2.0) * t242 * t505 + t242 * t509 + f64x8::splat(2.0) * t245 * t492 + f64x8::splat(2.0) * t498 * t501 - t512 * t514 + t516 * t91;
            let t519 = t239 * t518;
            let t521 = t125 * t410 + f64x8::splat(2.0) * t125 * t412 + t125 * t479 + f64x8::splat(2.0) * t236 * t488 - t236 * t519 - f64x8::splat(2.0) * t250 * t415 - f64x8::splat(2.0) * t250 * t481;
            let t524 = t62 * t66 * t521 * t254;
            let t525 = t252 * t252;
            let t527 = t127 * t127;
            let t528 = f64x8::splat(1.0) / t527;
            let t530 = t62 * t66 * t525 * t528;
            let t531 = -t310 - t315 - t322 + t349 + t357 + t361 + t365 + t372 - t383 - t391 + t524 - t530;
            let tv2rho20 = f64x8::splat(0.0022146941966666666) * t135 + f64x8::splat(2.0) * t160 - f64x8::splat(0.0003662289461201309) * t165 - f64x8::splat(1.1696447245269292) * t178 + f64x8::splat(2.0) * t256 + v_rho * t531;
            acc_v2rho2 = tv2rho20;
            let t534 = t182 * t39;
            let t535 = t534 * t262;
            let t538 = t187 * t39;
            let t540 = t71 * t1 * t3;
            let t541 = t538 * t540;
            let t542 = t190 * t265;
            let t546 = t233 * t90 * t124;
            let t547 = t79 * t546;
            let t550 = t79 * t118;
            let t551 = t90 * t238;
            let t552 = t551 * t249;
            let t553 = t550 * t552;
            let t556 = t194 * t287;
            let t559 = t534 * t71 * t270;
            let t561 = t104 * t104;
            let t562 = f64x8::splat(1.0) / t561;
            let t564 = t562 * t276 * t78;
            let t568 = f64x8::splat(1.0) / t21 / t561;
            let t570 = t568 * t276 * t20;
            let t571 = t5 * t77;
            let t572 = t571 * t280;
            let t576 = f64x8::splat(1.0) / t442 / t64;
            let t578 = t78 * t205;
            let t579 = t274 * t576 * t578;
            let t581 = t219 * t60 * t97;
            let t582 = t211 * t581;
            let t585 = t200 * t283;
            let t588 = t210 * v_sigma;
            let t589 = t588 * t106;
            let t590 = t208 * t589;
            let t595 = -f64x8::splat(7.0) / f64x8::splat(288.0) * t559 - f64x8::splat(0.0014130464632949138) * t564 * t280 - f64x8::splat(3.364396341178366e-05) * t570 * t572 + f64x8::splat(0.00020186378047070194) * t579 * t582 + t585 * t115 / f64x8::splat(1536.0) + t590 * t222 / f64x8::splat(1536.0) - f64x8::splat(7.0) / f64x8::splat(2304.0) * t284 * t230;
            let t596 = t83 * t595;
            let t598 = t288 * t90;
            let t603 = t486 * t296 * t249;
            let t608 = t190 * t290;
            let t611 = t109 * t64;
            let t612 = f64x8::splat(1.0) / t611;
            let t614 = t261 * t612 * t270;
            let t615 = t578 * t210;
            let t616 = t60 * t97;
            let t617 = t243 * t616;
            let t618 = t615 * t617;
            let t621 = t279 * t233;
            let t622 = t79 * t621;
            let t626 = t287 * t219;
            let t627 = t626 * t244;
            let t629 = t100 * t595;
            let t631 = -f64x8::splat(0.019190897446562643) * t535 * t291 - f64x8::splat(0.0041123351671205665) * t541 * t608 + f64x8::splat(0.008224670334241133) * t614 * t618 + f64x8::splat(0.008224670334241133) * t263 * t622 + t200 * t294 + t242 * t627 + t91 * t629;
            let t632 = t239 * t631;
            let t634 = -f64x8::splat(0.019190897446562643) * t535 * t266 - f64x8::splat(0.0041123351671205665) * t541 * t542 + f64x8::splat(0.008224670334241133) * t263 * t547 - f64x8::splat(0.008224670334241133) * t263 * t553 + t556 * t125 + t596 * t125 - t598 * t250 - t415 * t297 - t481 * t297 + f64x8::splat(2.0) * t236 * t603 - t236 * t632;
            let t635 = t66 * t634;
            let t636 = t635 * t254;
            let t638 = t528 * t252;
            let tv2rhosigma0 = -t260 * t300 * t638 + t260 * t636 + t301 * t62;
            acc_v2rhosigma = tv2rhosigma0;
            let t642 = t287 * t90 * t124;
            let t643 = t79 * t642;
            let t646 = t551 * t296;
            let t647 = t550 * t646;
            let t650 = t279 * v_sigma;
            let t653 = t100 * t106;
            let t656 = t107 * t110 * t114;
            let t659 = f64x8::splat(0.0008074551218828078) * t278 * t650 + t91 * t653 * t656 / f64x8::splat(1536.0);
            let t660 = t83 * t659;
            let t664 = t296 * t296;
            let t665 = t486 * t664;
            let t668 = t279 * t287;
            let t669 = t79 * t668;
            let t672 = t100 * t659;
            let t674 = f64x8::splat(0.016449340668482266) * t263 * t669 + t91 * t672;
            let t675 = t239 * t674;
            let t677 = f64x8::splat(0.016449340668482266) * t263 * t643 - f64x8::splat(0.016449340668482266) * t263 * t647 + t660 * t125 - f64x8::splat(2.0) * t598 * t297 + f64x8::splat(2.0) * t236 * t665 - t236 * t675;
            let t678 = t66 * t677;
            let t679 = t678 * t254;
            let t681 = t299 * t299;
            let t682 = t66 * t681;
            let t683 = t682 * t528;
            let tv2sigma20 = t260 * t679 - t260 * t683;
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
