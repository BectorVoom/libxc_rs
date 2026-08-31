//! MGGA_C_RSCAN vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_rscan.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_rscan_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
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
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        {
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t4 = (simd::cbrt(t3));
            let t5 = t2 * t4;
            let t6 = f64x8::splat(M_CBRT4);
            let t7 = t6 * t6;
            let t8 = (simd::cbrt(v_rho));
            let t11 = t5 * t7 / t8;
            let t13 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t11;
            let t14 = ((t11).sqrt());
            let t17 = ((t11) * (t11).sqrt());
            let t19 = t2 * t2;
            let t20 = t4 * t4;
            let t21 = t19 * t20;
            let t22 = t8 * t8;
            let t23 = f64x8::splat(1.0) / t22;
            let t25 = t21 * t6 * t23;
            let t27 = f64x8::splat(3.79785) * t14 + f64x8::splat(0.8969) * t11 + f64x8::splat(0.204775) * t17 + f64x8::splat(0.123235) * t25;
            let t30 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t27;
            let t31 = (simd::ln(t30));
            let t33 = f64x8::splat(0.0621814) * t13 * t31;
            let t34 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t35 = (simd::cbrt(zeta_threshold));
            let t37 = ((t34).select(t35 * zeta_threshold, f64x8::splat(1.0)));
            let t39 = f64x8::splat(2.0) * t37 - f64x8::splat(2.0);
            let t40 = f64x8::splat(M_CBRT2);
            let t41 = t40 - f64x8::splat(1.0);
            let t43 = f64x8::splat(1.0) / t41 / f64x8::splat(2.0);
            let t44 = t39 * t43;
            let t46 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t11;
            let t51 = f64x8::splat(5.1785) * t14 + f64x8::splat(0.905775) * t11 + f64x8::splat(0.1100325) * t17 + f64x8::splat(0.1241775) * t25;
            let t54 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t51;
            let t55 = (simd::ln(t54));
            let t58 = f64x8::splat(0.0197516734986138) * t44 * t46 * t55;
            let t59 = (simd::ln(f64x8::splat(2.0)));
            let t60 = f64x8::splat(1.0) - t59;
            let t61 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t63 = t60 / t61;
            let t64 = t35 * t35;
            let t65 = ((t34).select(t64, f64x8::splat(1.0)));
            let t66 = t65 * t65;
            let t67 = t66 * t65;
            let t69 = f64x8::splat(1.0) + f64x8::splat(0.025) * t11;
            let t71 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t11;
            let t72 = f64x8::splat(1.0) / t71;
            let t73 = t69 * t72;
            let t74 = f64x8::splat(1.0) / t60;
            let t77 = f64x8::splat(1.0) / t67;
            let t78 = t61 * t77;
            let t80 = (simd::exp(-(-t33 + t58) * t74 * t78));
            let t81 = t80 - f64x8::splat(1.0);
            let t82 = f64x8::splat(1.0) / t81;
            let t83 = t74 * t82;
            let t84 = t83 * v_sigma;
            let t85 = t73 * t84;
            let t86 = v_rho * v_rho;
            let t87 = t8 * t86;
            let t88 = f64x8::splat(1.0) / t87;
            let t89 = t88 * t40;
            let t90 = f64x8::splat(1.0) / t66;
            let t92 = f64x8::splat(1.0) / t4;
            let t94 = t19 * t92 * t6;
            let t98 = f64x8::splat(1.0) + f64x8::splat(0.027439371595564633) * t85 * t89 * t90 * t94;
            let t99 = ((t98).sqrt().sqrt());
            let t101 = f64x8::splat(1.0) - f64x8::splat(1.0) / t99;
            let t104 = f64x8::splat(1.0) + f64x8::splat(1.0) * t101 * t81;
            let t105 = (simd::ln(t104));
            let t107 = t63 * t67 * t105;
            let t108 = t86 * t86;
            let t109 = t108 * v_rho;
            let t110 = t22 * v_rho;
            let t111 = f64x8::splat(1.0) / t110;
            let t113 = t22 * t86;
            let t114 = f64x8::splat(1.0) / t113;
            let t117 = v_tau * t111 - v_sigma * t114 / f64x8::splat(8.0);
            let t118 = (f64x8::splat(0.0)).simd_lt(t117);
            let t119 = ((t118).select(t117, f64x8::splat(0.0)));
            let t120 = t119 * t119;
            let t121 = t120 * t119;
            let t122 = t109 * t121;
            let t123 = f64x8::splat(M_CBRT6);
            let t124 = t123 * t123;
            let t125 = (simd::cbrt(t61));
            let t126 = t125 * t125;
            let t127 = t124 * t126;
            let t130 = t40 * t40;
            let t132 = f64x8::splat(3.0) / f64x8::splat(10.0) * t127 * t110 + f64x8::splat(0.0001) * t130;
            let t133 = t132 * t132;
            let t134 = t133 * t132;
            let t135 = f64x8::splat(1.0) / t134;
            let t136 = t86 * v_rho;
            let t137 = t8 * t136;
            let t140 = f64x8::splat(1.0) / t133 * t40;
            let t143 = f64x8::splat(2.0) * t137 * t120 * t140 + f64x8::splat(0.001);
            let t144 = f64x8::splat(1.0) / t143;
            let t145 = t135 * t144;
            let t147 = f64x8::splat(4.0) * t122 * t145;
            let t148 = (t147).simd_le(f64x8::splat(2.5));
            let t149 = (f64x8::splat(2.5)).simd_lt(t147);
            let t150 = ((t149).select(f64x8::splat(2.5), t147));
            let t152 = t150 * t150;
            let t154 = t152 * t150;
            let t156 = t152 * t152;
            let t158 = t156 * t150;
            let t160 = t156 * t152;
            let t165 = ((t149).select(t147, f64x8::splat(2.5)));
            let t166 = f64x8::splat(1.0) - t165;
            let t169 = (simd::exp(f64x8::splat(1.5) / t166));
            let t171 = ((t148).select(f64x8::splat(1.0) - f64x8::splat(0.64) * t150 - f64x8::splat(0.4352) * t152 - f64x8::splat(1.535685604549) * t154 + f64x8::splat(3.061560252175) * t156 - f64x8::splat(1.915710236206) * t158 + f64x8::splat(0.516884468372) * t160 - f64x8::splat(0.051848879792) * t156 * t154, -f64x8::splat(0.7) * t169));
            let t174 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t14 + f64x8::splat(0.03138525) * t11;
            let t175 = f64x8::splat(1.0) / t174;
            let t178 = (simd::exp(f64x8::splat(1.0) * t175));
            let t179 = t178 - f64x8::splat(1.0);
            let t180 = f64x8::splat(1.0) / t126;
            let t181 = t123 * t180;
            let t182 = t130 * v_sigma;
            let t186 = f64x8::splat(1.0) + f64x8::splat(0.02133764210437636) * t181 * t182 * t114;
            let t187 = ((t186).sqrt().sqrt());
            let t189 = f64x8::splat(1.0) - f64x8::splat(1.0) / t187;
            let t191 = t179 * t189 + f64x8::splat(1.0);
            let t192 = (simd::ln(t191));
            let t198 = f64x8::splat(1.0) - f64x8::splat(2.363) * t41 * t39 * t43;
            let t200 = (-f64x8::splat(0.0285764) * t175 + f64x8::splat(0.0285764) * t192) * t198 + t33 - t58 - t107;
            let t201 = t171 * t200;
            let tzk0 = -t33 + t58 + t107 + t201;
            acc_zk = tzk0;
            let t202 = t8 * v_rho;
            let t203 = f64x8::splat(1.0) / t202;
            let t204 = t7 * t203;
            let t206 = t5 * t204 * t31;
            let t207 = f64x8::splat(0.0011073470983333333) * t206;
            let t208 = t27 * t27;
            let t209 = f64x8::splat(1.0) / t208;
            let t210 = t13 * t209;
            let t212 = f64x8::splat(1.0) / t14 * t2;
            let t213 = t4 * t7;
            let t214 = t213 * t203;
            let t215 = t212 * t214;
            let t217 = t5 * t204;
            let t219 = ((t11).sqrt());
            let t220 = t219 * t2;
            let t221 = t220 * t214;
            let t224 = t21 * t6 * t111;
            let t226 = -f64x8::splat(0.632975) * t215 - f64x8::splat(0.29896666666666666) * t217 - f64x8::splat(0.1023875) * t221 - f64x8::splat(0.08215666666666667) * t224;
            let t227 = f64x8::splat(1.0) / t30;
            let t228 = t226 * t227;
            let t229 = t210 * t228;
            let t230 = f64x8::splat(1.0) * t229;
            let t231 = t44 * t2;
            let t234 = t231 * t213 * t203 * t55;
            let t235 = f64x8::splat(0.00018311447306006544) * t234;
            let t236 = t44 * t46;
            let t237 = t51 * t51;
            let t238 = f64x8::splat(1.0) / t237;
            let t243 = -f64x8::splat(0.8630833333333333) * t215 - f64x8::splat(0.301925) * t217 - f64x8::splat(0.05501625) * t221 - f64x8::splat(0.082785) * t224;
            let t245 = f64x8::splat(1.0) / t54;
            let t246 = t238 * t243 * t245;
            let t247 = t236 * t246;
            let t248 = f64x8::splat(0.5848223622634646) * t247;
            let t250 = f64x8::splat(1.0) / t99 / t98;
            let t251 = t22 * t136;
            let t252 = f64x8::splat(1.0) / t251;
            let t253 = t252 * t72;
            let t256 = t40 * t90;
            let t257 = t82 * v_sigma * t256;
            let t260 = t71 * t71;
            let t261 = f64x8::splat(1.0) / t260;
            let t262 = t69 * t261;
            let t263 = t262 * t83;
            let t264 = v_sigma * t252;
            let t268 = t60 * t60;
            let t269 = f64x8::splat(1.0) / t268;
            let t270 = t73 * t269;
            let t271 = t81 * t81;
            let t272 = f64x8::splat(1.0) / t271;
            let t273 = t272 * v_sigma;
            let t274 = t273 * t89;
            let t275 = t270 * t274;
            let t276 = t66 * t66;
            let t278 = f64x8::splat(1.0) / t276 / t65;
            let t279 = t278 * t19;
            let t280 = t279 * t92;
            let t281 = t207 + t230 - t235 - t248;
            let t283 = t61 * t80;
            let t284 = t6 * t281 * t283;
            let t285 = t280 * t284;
            let t288 = f64x8::splat(1.0) / t137;
            let t289 = t288 * t40;
            let t294 = -f64x8::splat(0.002743937159556463) * t253 * t74 * t257 + f64x8::splat(0.004878720269691391) * t263 * t264 * t256 + f64x8::splat(0.027439371595564633) * t275 * t285 - f64x8::splat(0.0640252003896508) * t85 * t289 * t90 * t94;
            let t295 = t250 * t294;
            let t300 = t78 * t80;
            let t303 = f64x8::splat(0.25) * t295 * t81 - f64x8::splat(1.0) * t101 * t281 * t74 * t300;
            let t305 = f64x8::splat(1.0) / t104;
            let t307 = t63 * t67 * t303 * t305;
            let t308 = t108 * t121;
            let t311 = t109 * t120;
            let t316 = ((t118).select(-f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau * t114 + t264 / f64x8::splat(3.0), f64x8::splat(0.0)));
            let t317 = t145 * t316;
            let t320 = t22 * t109;
            let t321 = t320 * t121;
            let t322 = t133 * t133;
            let t323 = f64x8::splat(1.0) / t322;
            let t324 = t321 * t323;
            let t326 = t144 * t124 * t126;
            let t329 = t143 * t143;
            let t330 = f64x8::splat(1.0) / t329;
            let t331 = t135 * t330;
            let t335 = t137 * t119;
            let t336 = t140 * t316;
            let t339 = t108 * t120;
            let t340 = t339 * t135;
            let t342 = t40 * t124 * t126;
            let t345 = f64x8::splat(20.0) / f64x8::splat(3.0) * t87 * t120 * t140 + f64x8::splat(4.0) * t335 * t336 - f64x8::splat(2.0) * t340 * t342;
            let t346 = t331 * t345;
            let t349 = -f64x8::splat(4.0) * t122 * t346 + f64x8::splat(20.0) * t308 * t145 + f64x8::splat(12.0) * t311 * t317 - f64x8::splat(6.0) * t324 * t326;
            let t350 = ((t149).select(f64x8::splat(0.0), t349));
            let t352 = t150 * t350;
            let t354 = t152 * t350;
            let t356 = t154 * t350;
            let t358 = t156 * t350;
            let t360 = t158 * t350;
            let t365 = t166 * t166;
            let t366 = f64x8::splat(1.0) / t365;
            let t367 = ((t149).select(t349, f64x8::splat(0.0)));
            let t371 = ((t148).select(-f64x8::splat(0.64) * t350 - f64x8::splat(0.8704) * t352 - f64x8::splat(4.607056813647) * t354 + f64x8::splat(12.2462410087) * t356 - f64x8::splat(9.57855118103) * t358 + f64x8::splat(3.101306810232) * t360 - f64x8::splat(0.362942158544) * t160 * t350, -f64x8::splat(1.05) * t366 * t367 * t169));
            let t372 = t371 * t200;
            let t373 = t174 * t174;
            let t374 = f64x8::splat(1.0) / t373;
            let t377 = -f64x8::splat(0.007408333333333334) * t215 - f64x8::splat(0.01046175) * t217;
            let t378 = t374 * t377;
            let t380 = t178 * t189;
            let t384 = f64x8::splat(1.0) / t187 / t186;
            let t385 = t179 * t384;
            let t386 = t385 * t123;
            let t387 = t180 * t130;
            let t391 = -f64x8::splat(1.0) * t378 * t380 - f64x8::splat(0.014225094736250906) * t386 * t387 * t264;
            let t392 = f64x8::splat(1.0) / t191;
            let t397 = (f64x8::splat(0.0285764) * t378 + f64x8::splat(0.0285764) * t391 * t392) * t198 - t207 - t230 + t235 + t248 - t307;
            let t398 = t171 * t397;
            let tvrho0 = -t33 + t58 + t107 + t201 + v_rho * (t207 + t230 - t235 - t248 + t307 + t372 + t398);
            acc_vrho = tvrho0;
            let t401 = t65 * t250;
            let t402 = t73 * t88;
            let t403 = t401 * t402;
            let t404 = t40 * t19;
            let t405 = t92 * t6;
            let t406 = t405 * t305;
            let t407 = t404 * t406;
            let t409 = f64x8::splat(0.0006950474021161377) * t403 * t407;
            let t411 = ((t118).select(-t114 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t412 = t145 * t411;
            let t415 = t108 * t108;
            let t416 = t8 * t415;
            let t417 = t120 * t120;
            let t418 = t416 * t417;
            let t420 = f64x8::splat(1.0) / t322 / t132;
            let t421 = t418 * t420;
            let t422 = t330 * t40;
            let t423 = t422 * t411;
            let t426 = f64x8::splat(12.0) * t311 * t412 - f64x8::splat(16.0) * t421 * t423;
            let t427 = ((t149).select(f64x8::splat(0.0), t426));
            let t429 = t150 * t427;
            let t431 = t152 * t427;
            let t433 = t154 * t427;
            let t435 = t156 * t427;
            let t437 = t158 * t427;
            let t442 = ((t149).select(t426, f64x8::splat(0.0)));
            let t446 = ((t148).select(-f64x8::splat(0.64) * t427 - f64x8::splat(0.8704) * t429 - f64x8::splat(4.607056813647) * t431 + f64x8::splat(12.2462410087) * t433 - f64x8::splat(9.57855118103) * t435 + f64x8::splat(3.101306810232) * t437 - f64x8::splat(0.362942158544) * t160 * t427, -f64x8::splat(1.05) * t366 * t442 * t169));
            let t447 = t446 * t200;
            let t448 = t385 * t181;
            let t449 = t130 * t114;
            let t450 = t392 * t198;
            let t454 = f64x8::splat(0.00015243824895787514) * t448 * t449 * t450 - t409;
            let t455 = t171 * t454;
            let tvsigma0 = v_rho * (t409 + t447 + t455);
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t457 = ((t118).select(t111, f64x8::splat(0.0)));
            let t458 = t145 * t457;
            let t461 = t422 * t457;
            let t464 = f64x8::splat(12.0) * t311 * t458 - f64x8::splat(16.0) * t421 * t461;
            let t465 = ((t149).select(f64x8::splat(0.0), t464));
            let t467 = t150 * t465;
            let t469 = t152 * t465;
            let t471 = t154 * t465;
            let t473 = t156 * t465;
            let t475 = t158 * t465;
            let t480 = ((t149).select(t464, f64x8::splat(0.0)));
            let t484 = ((t148).select(-f64x8::splat(0.64) * t465 - f64x8::splat(0.8704) * t467 - f64x8::splat(4.607056813647) * t469 + f64x8::splat(12.2462410087) * t471 - f64x8::splat(9.57855118103) * t473 + f64x8::splat(3.101306810232) * t475 - f64x8::splat(0.362942158544) * t160 * t465, -f64x8::splat(1.05) * t366 * t480 * t169));
            let t485 = v_rho * t484;
            let tvtau0 = t485 * t200;
            acc_vtau = tvtau0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vlapl.into(); vlapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vtau.into(); vtau[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
