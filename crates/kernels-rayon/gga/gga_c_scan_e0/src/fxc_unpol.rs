//! GGA_C_SCAN_E0 fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_scan_e0.c`
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
pub fn gga_c_scan_e0_fxc_unpol(
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
            let t62 = t59 / t60;
            let t63 = t34 * t34;
            let t64 = ((t33).select(t63, f64x8::splat(1.0)));
            let t65 = t64 * t64;
            let t66 = t65 * t64;
            let t68 = f64x8::splat(1.0) + f64x8::splat(0.025) * t10;
            let t70 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t10;
            let t71 = f64x8::splat(1.0) / t70;
            let t72 = t68 * t71;
            let t73 = f64x8::splat(1.0) / t59;
            let t76 = f64x8::splat(1.0) / t66;
            let t77 = t60 * t76;
            let t79 = (simd::exp(-(-t32 + t57) * t73 * t77));
            let t80 = t79 - f64x8::splat(1.0);
            let t81 = f64x8::splat(1.0) / t80;
            let t82 = t73 * t81;
            let t83 = t82 * v_sigma;
            let t84 = t72 * t83;
            let t85 = v_rho * v_rho;
            let t87 = f64x8::splat(1.0) / t7 / t85;
            let t88 = t87 * t39;
            let t89 = f64x8::splat(1.0) / t65;
            let t91 = f64x8::splat(1.0) / t3;
            let t93 = t18 * t91 * t5;
            let t97 = f64x8::splat(1.0) + f64x8::splat(0.027439371595564633) * t84 * t88 * t89 * t93;
            let t98 = ((t97).sqrt().sqrt());
            let t100 = f64x8::splat(1.0) - f64x8::splat(1.0) / t98;
            let t103 = f64x8::splat(1.0) + f64x8::splat(1.0) * t100 * t80;
            let t104 = (simd::ln(t103));
            let t106 = t62 * t66 * t104;
            let tzk0 = -t32 + t57 + t106;
            acc_zk = tzk0;
            let t108 = f64x8::splat(1.0) / t7 / v_rho;
            let t109 = t6 * t108;
            let t111 = t4 * t109 * t30;
            let t112 = f64x8::splat(0.0011073470983333333) * t111;
            let t113 = t26 * t26;
            let t114 = f64x8::splat(1.0) / t113;
            let t115 = t12 * t114;
            let t117 = f64x8::splat(1.0) / t13 * t1;
            let t118 = t3 * t6;
            let t119 = t118 * t108;
            let t120 = t117 * t119;
            let t122 = t4 * t109;
            let t124 = ((t10).sqrt());
            let t125 = t124 * t1;
            let t126 = t125 * t119;
            let t131 = t20 * t5 / t21 / v_rho;
            let t133 = -f64x8::splat(0.632975) * t120 - f64x8::splat(0.29896666666666666) * t122 - f64x8::splat(0.1023875) * t126 - f64x8::splat(0.08215666666666667) * t131;
            let t134 = f64x8::splat(1.0) / t29;
            let t135 = t133 * t134;
            let t136 = t115 * t135;
            let t137 = f64x8::splat(1.0) * t136;
            let t138 = t43 * t1;
            let t141 = t138 * t118 * t108 * t54;
            let t142 = f64x8::splat(0.00018311447306006544) * t141;
            let t143 = t43 * t45;
            let t144 = t50 * t50;
            let t145 = f64x8::splat(1.0) / t144;
            let t150 = -f64x8::splat(0.8630833333333333) * t120 - f64x8::splat(0.301925) * t122 - f64x8::splat(0.05501625) * t126 - f64x8::splat(0.082785) * t131;
            let t152 = f64x8::splat(1.0) / t53;
            let t153 = t145 * t150 * t152;
            let t154 = t143 * t153;
            let t155 = f64x8::splat(0.5848223622634646) * t154;
            let t157 = f64x8::splat(1.0) / t98 / t97;
            let t158 = t85 * v_rho;
            let t160 = f64x8::splat(1.0) / t21 / t158;
            let t161 = t160 * t71;
            let t164 = t39 * t89;
            let t165 = t81 * v_sigma * t164;
            let t168 = t70 * t70;
            let t169 = f64x8::splat(1.0) / t168;
            let t170 = t68 * t169;
            let t171 = t170 * t82;
            let t176 = t59 * t59;
            let t177 = f64x8::splat(1.0) / t176;
            let t178 = t72 * t177;
            let t179 = t80 * t80;
            let t180 = f64x8::splat(1.0) / t179;
            let t181 = t180 * v_sigma;
            let t182 = t181 * t88;
            let t183 = t178 * t182;
            let t184 = t65 * t65;
            let t186 = f64x8::splat(1.0) / t184 / t64;
            let t187 = t186 * t18;
            let t188 = t187 * t91;
            let t189 = t112 + t137 - t142 - t155;
            let t191 = t60 * t79;
            let t192 = t5 * t189 * t191;
            let t193 = t188 * t192;
            let t197 = f64x8::splat(1.0) / t7 / t158;
            let t198 = t197 * t39;
            let t203 = -f64x8::splat(0.002743937159556463) * t161 * t73 * t165 + f64x8::splat(0.004878720269691391) * t171 * v_sigma * t160 * t164 + f64x8::splat(0.027439371595564633) * t183 * t193 - f64x8::splat(0.0640252003896508) * t84 * t198 * t89 * t93;
            let t204 = t157 * t203;
            let t209 = t77 * t79;
            let t212 = f64x8::splat(0.25) * t204 * t80 - f64x8::splat(1.0) * t100 * t189 * t73 * t209;
            let t214 = f64x8::splat(1.0) / t103;
            let t216 = t62 * t66 * t212 * t214;
            let tvrho0 = -t32 + t57 + t106 + v_rho * (t112 + t137 - t142 - t155 + t216);
            acc_vrho = tvrho0;
            let t219 = t108 * t64;
            let t220 = t157 * t68;
            let t222 = t219 * t220 * t71;
            let t223 = t39 * t18;
            let t224 = t91 * t5;
            let t225 = t224 * t214;
            let t226 = t223 * t225;
            let tvsigma0 = f64x8::splat(0.0006950474021161377) * t222 * t226;
            acc_vsigma = tvsigma0;
            let t233 = t6 * t87;
            let t235 = t4 * t233 * t30;
            let t236 = f64x8::splat(0.0014764627977777779) * t235;
            let t237 = t4 * t6;
            let t238 = t108 * t114;
            let t240 = t237 * t238 * t135;
            let t241 = f64x8::splat(0.035616666666666665) * t240;
            let t242 = t113 * t26;
            let t243 = f64x8::splat(1.0) / t242;
            let t244 = t12 * t243;
            let t245 = t133 * t133;
            let t246 = t245 * t134;
            let t247 = t244 * t246;
            let t248 = f64x8::splat(2.0) * t247;
            let t251 = f64x8::splat(1.0) / t13 / t10 * t18;
            let t252 = t19 * t5;
            let t254 = f64x8::splat(1.0) / t21 / t85;
            let t255 = t252 * t254;
            let t256 = t251 * t255;
            let t258 = t118 * t87;
            let t259 = t117 * t258;
            let t261 = t4 * t233;
            let t263 = f64x8::splat(1.0)/((t10).sqrt());
            let t264 = t263 * t18;
            let t265 = t264 * t255;
            let t267 = t125 * t258;
            let t270 = t20 * t5 * t254;
            let t272 = -f64x8::splat(0.4219833333333333) * t256 + f64x8::splat(0.8439666666666666) * t259 + f64x8::splat(0.3986222222222222) * t261 + f64x8::splat(0.06825833333333334) * t265 + f64x8::splat(0.13651666666666668) * t267 + f64x8::splat(0.1369277777777778) * t270;
            let t273 = t272 * t134;
            let t274 = t115 * t273;
            let t275 = f64x8::splat(1.0) * t274;
            let t276 = t113 * t113;
            let t277 = f64x8::splat(1.0) / t276;
            let t278 = t12 * t277;
            let t279 = t29 * t29;
            let t280 = f64x8::splat(1.0) / t279;
            let t281 = t245 * t280;
            let t282 = t278 * t281;
            let t283 = f64x8::splat(16.081979498692537) * t282;
            let t286 = t138 * t118 * t87 * t54;
            let t287 = f64x8::splat(0.00024415263074675396) * t286;
            let t288 = t43 * t4;
            let t290 = t288 * t109 * t153;
            let t291 = f64x8::splat(0.01084358130030174) * t290;
            let t292 = t144 * t50;
            let t293 = f64x8::splat(1.0) / t292;
            let t294 = t150 * t150;
            let t296 = t293 * t294 * t152;
            let t297 = t143 * t296;
            let t298 = f64x8::splat(1.1696447245269292) * t297;
            let t305 = -f64x8::splat(0.5753888888888888) * t256 + f64x8::splat(1.1507777777777777) * t259 + f64x8::splat(0.4025666666666667) * t261 + f64x8::splat(0.0366775) * t265 + f64x8::splat(0.073355) * t267 + f64x8::splat(0.137975) * t270;
            let t307 = t145 * t305 * t152;
            let t308 = t143 * t307;
            let t309 = f64x8::splat(0.5848223622634646) * t308;
            let t310 = t144 * t144;
            let t311 = f64x8::splat(1.0) / t310;
            let t312 = t311 * t294;
            let t313 = t53 * t53;
            let t314 = f64x8::splat(1.0) / t313;
            let t315 = t312 * t314;
            let t316 = t143 * t315;
            let t317 = f64x8::splat(17.315859105681465) * t316;
            let t318 = t97 * t97;
            let t320 = f64x8::splat(1.0) / t98 / t318;
            let t321 = t203 * t203;
            let t322 = t320 * t321;
            let t325 = t85 * t85;
            let t327 = f64x8::splat(1.0) / t21 / t325;
            let t328 = t327 * t71;
            let t332 = t325 * v_rho;
            let t333 = f64x8::splat(1.0) / t332;
            let t334 = t333 * t169;
            let t336 = t164 * t237;
            let t339 = t177 * t180;
            let t340 = t339 * v_sigma;
            let t341 = t161 * t340;
            let t342 = t39 * t186;
            let t344 = t189 * t60 * t79;
            let t345 = t342 * t344;
            let t349 = f64x8::splat(1.0) / t168 / t70;
            let t350 = t68 * t349;
            let t351 = t350 * t83;
            let t352 = t333 * t39;
            let t357 = t170 * t340;
            let t358 = t160 * t39;
            let t359 = t358 * t186;
            let t367 = t161 * t339;
            let t368 = v_sigma * t39;
            let t370 = t186 * t189 * t79;
            let t371 = t368 * t370;
            let t378 = f64x8::splat(1.0) / t176 / t59;
            let t379 = t72 * t378;
            let t381 = f64x8::splat(1.0) / t179 / t80;
            let t382 = t381 * v_sigma;
            let t383 = t382 * t88;
            let t384 = t379 * t383;
            let t385 = t184 * t184;
            let t386 = f64x8::splat(1.0) / t385;
            let t387 = t386 * t18;
            let t388 = t387 * t91;
            let t389 = t189 * t189;
            let t390 = t5 * t389;
            let t391 = t60 * t60;
            let t392 = t79 * t79;
            let t393 = t391 * t392;
            let t395 = t388 * t390 * t393;
            let t398 = t181 * t198;
            let t399 = t178 * t398;
            let t402 = -t236 - t241 - t248 + t275 + t283 + t287 + t291 + t298 - t309 - t317;
            let t405 = t188 * t5 * t402 * t191;
            let t408 = t379 * t182;
            let t409 = t391 * t79;
            let t411 = t388 * t390 * t409;
            let t415 = f64x8::splat(1.0) / t7 / t325;
            let t416 = t415 * t39;
            let t421 = f64x8::splat(0.01646362295733878) * t328 * t73 * t165 - f64x8::splat(8.131200449485652e-05) * t334 * t83 * t336 - f64x8::splat(0.002743937159556463) * t341 * t345 + f64x8::splat(0.0001445727439918549) * t351 * t352 * t89 * t237 + f64x8::splat(0.004878720269691391) * t357 * t359 * t344 - f64x8::splat(0.02927232161814835) * t171 * v_sigma * t327 * t164 - f64x8::splat(0.027081574266271103) * t367 * t371 + f64x8::splat(0.04815103904543002) * t357 * t358 * t370 + f64x8::splat(0.054878743191129266) * t384 * t395 - f64x8::splat(0.1280504007793016) * t399 * t193 + f64x8::splat(0.027439371595564633) * t183 * t405 - f64x8::splat(0.027439371595564633) * t408 * t411 + f64x8::splat(0.21341733463216936) * t84 * t416 * t89 * t93;
            let t422 = t157 * t421;
            let t428 = t73 * t60 * t76 * t79;
            let t431 = t100 * t402;
            let t437 = t184 * t65;
            let t438 = f64x8::splat(1.0) / t437;
            let t439 = t391 * t438;
            let t440 = t439 * t79;
            let t443 = -f64x8::splat(0.3125) * t322 * t80 + f64x8::splat(0.25) * t422 * t80 - f64x8::splat(0.5) * t204 * t189 * t428 - f64x8::splat(1.0) * t431 * t73 * t209 + f64x8::splat(1.0) * t100 * t389 * t177 * t440;
            let t446 = t62 * t66 * t443 * t214;
            let t447 = t212 * t212;
            let t449 = t103 * t103;
            let t450 = f64x8::splat(1.0) / t449;
            let t452 = t62 * t66 * t447 * t450;
            let t453 = -t236 - t241 - t248 + t275 + t283 + t287 + t291 + t298 - t309 - t317 + t446 - t452;
            let tv2rho20 = f64x8::splat(0.0022146941966666666) * t111 + f64x8::splat(2.0) * t136 - f64x8::splat(0.0003662289461201309) * t141 - f64x8::splat(1.1696447245269292) * t154 + f64x8::splat(2.0) * t216 + v_rho * t453;
            acc_v2rho2 = tv2rho20;
            let t455 = t64 * t157;
            let t456 = t72 * t87;
            let t457 = t455 * t456;
            let t460 = t320 * t68;
            let t462 = t219 * t460 * t71;
            let t463 = t223 * t91;
            let t464 = t5 * t214;
            let t466 = t463 * t464 * t203;
            let t469 = t254 * t64;
            let t470 = t469 * t157;
            let t471 = t71 * t39;
            let t472 = t471 * t214;
            let t475 = t39 * t214;
            let t479 = t5 * t450;
            let t481 = t463 * t479 * t212;
            let tv2rhosigma0 = -f64x8::splat(0.0009267298694881837) * t457 * t226 - f64x8::splat(0.0008688092526451722) * t462 * t466 - f64x8::splat(6.950474021161377e-05) * t470 * t472 + f64x8::splat(0.00012357942809624928) * t470 * t170 * t475 - f64x8::splat(0.0006950474021161377) * t222 * t481;
            acc_v2rhosigma = tv2rhosigma0;
            let t484 = f64x8::splat(1.0) / t64;
            let t485 = t160 * t484;
            let t487 = t68 * t68;
            let t488 = t487 * t169;
            let t489 = t39 * t39;
            let t490 = t488 * t489;
            let t491 = t485 * t320 * t490;
            let t492 = f64x8::splat(1.0) / t19;
            let t493 = t1 * t492;
            let t494 = t493 * t6;
            let t495 = t214 * t73;
            let t496 = t495 * t81;
            let t497 = t494 * t496;
            let t500 = ((t97).sqrt());
            let t502 = f64x8::splat(1.0) / t500 / t318;
            let t503 = t502 * t487;
            let t504 = t503 * t169;
            let t506 = t489 * t1;
            let t508 = t6 * t450;
            let t510 = t506 * t492 * t508 * t73;
            let tv2sigma20 = -f64x8::splat(7.151873978698702e-05) * t491 * t497 - f64x8::splat(1.4303747957397403e-05) * t485 * t504 * t510;
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
