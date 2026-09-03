//! MGGA_X_BR89_EXPLICIT vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_br89_explicit.c`
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
pub fn mgga_x_br89_explicit_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_gamma = f64x8::splat(param_gamma);
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
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t5 = zeta_threshold - f64x8::splat(1.0);
            let t7 = ((t4).select(t5, (t4).select(-t5, f64x8::splat(0.0))));
            let t8 = f64x8::splat(1.0) + t7;
            let t10 = (simd::cbrt(zeta_threshold));
            let t12 = (simd::cbrt(t8));
            let t14 = (((t8).simd_le(zeta_threshold)).select(t10 * zeta_threshold, t12 * t8));
            let t15 = (simd::cbrt(v_rho));
            let t16 = t14 * t15;
            let t18 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t19 = f64x8::splat(1.0) / t18;
            let t20 = t16 * t19;
            let t21 = f64x8::splat(M_CBRT4);
            let t22 = f64x8::splat(M_CBRTPI);
            let t23 = t22 * t22;
            let t24 = f64x8::splat(M_CBRT2);
            let t25 = t24 * t24;
            let t26 = t15 * t15;
            let t28 = f64x8::splat(1.0) / t26 / v_rho;
            let t31 = param_gamma * v_tau;
            let t34 = param_gamma * v_sigma;
            let t35 = v_rho * v_rho;
            let t37 = f64x8::splat(1.0) / t26 / t35;
            let t41 = ((v_lapl * t28 / f64x8::splat(2.0) - f64x8::splat(2.0) * t31 * t28 + t34 * t37 / f64x8::splat(4.0)).abs());
            let t44 = (t25 * t41 / f64x8::splat(3.0)).simd_lt(f64x8::splat(5e-13));
            let t45 = v_lapl * t25;
            let t48 = t25 * t28;
            let t51 = t25 * t37;
            let t54 = t45 * t28 / f64x8::splat(6.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t31 * t48 + t34 * t51 / f64x8::splat(12.0);
            let t55 = (f64x8::splat(0.0)).simd_lt(t54);
            let t56 = ((t55).select(f64x8::splat(5e-13), -f64x8::splat(5e-13)));
            let t57 = ((t44).select(t56, t54));
            let t60 = f64x8::splat(2.0) / f64x8::splat(3.0) * t23 / t57;
            let t61 = (t60).simd_le(f64x8::splat(0.0));
            let t62 = (-f64x8::splat(5e-13)).simd_lt(t60);
            let t63 = ((t62).select(-f64x8::splat(5e-13), t60));
            let t65 = f64x8::splat(1.525525181200953) * t63 + f64x8::splat(0.4576575543602858);
            let t66 = (simd::atan(t65));
            let t67 = -t66 + f64x8::splat(0.4292036732051034);
            let t69 = t63 * t63;
            let t71 = t69 * t63;
            let t73 = t69 * t69;
            let t75 = t73 * t63;
            let t77 = f64x8::splat(0.7566445420735584) - f64x8::splat(2.636397787137096) * t63 + f64x8::splat(5.474515996423288) * t69 - f64x8::splat(12.65730812710829) * t71 + f64x8::splat(4.125058472512136) * t73 - f64x8::splat(30.42513395716384) * t75;
            let t78 = t67 * t77;
            let t84 = f64x8::splat(0.4771976183772063) - f64x8::splat(1.779981349455627) * t63 + f64x8::splat(3.843384186230215) * t69 - f64x8::splat(9.591205088051849) * t71 + f64x8::splat(2.173018028591672) * t73 - f64x8::splat(30.42513385160366) * t75;
            let t85 = f64x8::splat(1.0) / t84;
            let t87 = (f64x8::splat(5e-13)).simd_lt(t60);
            let t88 = ((t87).select(t60, f64x8::splat(5e-13)));
            let t90 = (simd::ln(f64x8::splat(1.0) / (f64x8::splat(2.085749716493756) * t88) + ((((f64x8::splat(1.0) / (f64x8::splat(2.085749716493756) * t88)) * (f64x8::splat(1.0) / (f64x8::splat(2.085749716493756) * t88))) + f64x8::splat(1.0)).sqrt())));
            let t91 = t90 + f64x8::splat(2.0);
            let t93 = t88 * t88;
            let t95 = t93 * t88;
            let t97 = t93 * t93;
            let t99 = t97 * t88;
            let t101 = f64x8::splat(4.435009886795587e-05) + f64x8::splat(0.5812865360445791) * t88 + f64x8::splat(66.7427645159406) * t93 + f64x8::splat(434.2678089722977) * t95 + f64x8::splat(824.7765766052239) * t97 + f64x8::splat(1657.965273158212) * t99;
            let t102 = t91 * t101;
            let t108 = f64x8::splat(3.347285060926091e-05) + f64x8::splat(0.4791793102397135) * t88 + f64x8::splat(62.39226833857424) * t93 + f64x8::splat(463.1481642793812) * t95 + f64x8::splat(785.2360350104029) * t97 + f64x8::splat(1657.962968223273) * t99;
            let t109 = f64x8::splat(1.0) / t108;
            let t111 = ((t61).select(t78 * t85, t102 * t109));
            let t113 = (simd::exp(t111 / f64x8::splat(3.0)));
            let t114 = t21 * t113;
            let t115 = (simd::exp(-t111));
            let t117 = f64x8::splat(1.0) + t111 / f64x8::splat(2.0);
            let t118 = t115 * t117;
            let t119 = f64x8::splat(1.0) - t118;
            let t120 = f64x8::splat(1.0) / t111;
            let t121 = t119 * t120;
            let t122 = t114 * t121;
            let t125 = ((t3).select(f64x8::splat(0.0), -t20 * t122 / f64x8::splat(4.0)));
            let tzk0 = f64x8::splat(2.0) * t125;
            acc_zk = tzk0;
            let t127 = t14 / t26;
            let t128 = t127 * t19;
            let t131 = t19 * t21;
            let t132 = t16 * t131;
            let t133 = t57 * t57;
            let t135 = t23 / t133;
            let t136 = ((t55).select(f64x8::splat(0.0), f64x8::splat(0.0)));
            let t143 = f64x8::splat(1.0) / t26 / t35 / v_rho;
            let t144 = t25 * t143;
            let t148 = ((t44).select(t136, -f64x8::splat(5.0) / f64x8::splat(18.0) * t45 * t37 + f64x8::splat(10.0) / f64x8::splat(9.0) * t31 * t51 - f64x8::splat(2.0) / f64x8::splat(9.0) * t34 * t144));
            let t150 = f64x8::splat(2.0) / f64x8::splat(3.0) * t135 * t148;
            let t151 = ((t62).select(f64x8::splat(0.0), -t150));
            let t152 = t65 * t65;
            let t153 = t152 + f64x8::splat(1.0);
            let t154 = f64x8::splat(1.0) / t153;
            let t155 = t151 * t154;
            let t156 = t77 * t85;
            let t160 = t63 * t151;
            let t162 = t69 * t151;
            let t164 = t71 * t151;
            let t166 = t73 * t151;
            let t168 = -f64x8::splat(2.636397787137096) * t151 + f64x8::splat(10.949031992846576) * t160 - f64x8::splat(37.97192438132487) * t162 + f64x8::splat(16.500233890048545) * t164 - f64x8::splat(152.1256697858192) * t166;
            let t169 = t67 * t168;
            let t171 = t84 * t84;
            let t172 = f64x8::splat(1.0) / t171;
            let t178 = -f64x8::splat(1.779981349455627) * t151 + f64x8::splat(7.68676837246043) * t160 - f64x8::splat(28.77361526415555) * t162 + f64x8::splat(8.692072114366688) * t164 - f64x8::splat(152.1256692580183) * t166;
            let t179 = t172 * t178;
            let t182 = ((t87).select(-t150, f64x8::splat(0.0)));
            let t183 = f64x8::splat(1.0) / t93;
            let t184 = t182 * t183;
            let t186 = f64x8::splat(1.0) + f64x8::splat(0.2298664631316238) * t183;
            let t187 = ((t186).sqrt());
            let t188 = f64x8::splat(1.0) / t187;
            let t189 = t188 * t101;
            let t190 = t189 * t109;
            let t194 = t88 * t182;
            let t196 = t93 * t182;
            let t198 = t95 * t182;
            let t200 = t97 * t182;
            let t202 = f64x8::splat(0.5812865360445791) * t182 + f64x8::splat(133.4855290318812) * t194 + f64x8::splat(1302.8034269168932) * t196 + f64x8::splat(3299.1063064208956) * t198 + f64x8::splat(8289.82636579106) * t200;
            let t203 = t91 * t202;
            let t205 = t108 * t108;
            let t206 = f64x8::splat(1.0) / t205;
            let t212 = f64x8::splat(0.4791793102397135) * t182 + f64x8::splat(124.78453667714848) * t194 + f64x8::splat(1389.4444928381436) * t196 + f64x8::splat(3140.9441400416117) * t198 + f64x8::splat(8289.814841116366) * t200;
            let t213 = t206 * t212;
            let t216 = ((t61).select(-f64x8::splat(1.525525181200953) * t155 * t156 + t169 * t85 - t78 * t179, -f64x8::splat(0.47944391030820677) * t184 * t190 + t203 * t109 - t102 * t213));
            let t217 = t216 * t113;
            let t218 = t217 * t121;
            let t221 = t216 * t115;
            let t222 = t221 * t117;
            let t224 = t222 - t221 / f64x8::splat(2.0);
            let t225 = t224 * t120;
            let t226 = t114 * t225;
            let t229 = t113 * t119;
            let t230 = t111 * t111;
            let t231 = f64x8::splat(1.0) / t230;
            let t232 = t231 * t216;
            let t233 = t229 * t232;
            let t237 = ((t3).select(f64x8::splat(0.0), -t128 * t122 / f64x8::splat(12.0) - t132 * t218 / f64x8::splat(12.0) - t20 * t226 / f64x8::splat(4.0) + t132 * t233 / f64x8::splat(4.0)));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t237 + f64x8::splat(2.0) * t125;
            acc_vrho = tvrho0;
            let t240 = param_gamma * t25;
            let t241 = t240 * t37;
            let t243 = ((t44).select(t136, t241 / f64x8::splat(12.0)));
            let t245 = f64x8::splat(2.0) / f64x8::splat(3.0) * t135 * t243;
            let t246 = ((t62).select(f64x8::splat(0.0), -t245));
            let t247 = t246 * t154;
            let t251 = t63 * t246;
            let t253 = t69 * t246;
            let t255 = t71 * t246;
            let t257 = t73 * t246;
            let t259 = -f64x8::splat(2.636397787137096) * t246 + f64x8::splat(10.949031992846576) * t251 - f64x8::splat(37.97192438132487) * t253 + f64x8::splat(16.500233890048545) * t255 - f64x8::splat(152.1256697858192) * t257;
            let t260 = t67 * t259;
            let t267 = -f64x8::splat(1.779981349455627) * t246 + f64x8::splat(7.68676837246043) * t251 - f64x8::splat(28.77361526415555) * t253 + f64x8::splat(8.692072114366688) * t255 - f64x8::splat(152.1256692580183) * t257;
            let t268 = t172 * t267;
            let t271 = ((t87).select(-t245, f64x8::splat(0.0)));
            let t272 = t271 * t183;
            let t276 = t88 * t271;
            let t278 = t93 * t271;
            let t280 = t95 * t271;
            let t282 = t97 * t271;
            let t284 = f64x8::splat(0.5812865360445791) * t271 + f64x8::splat(133.4855290318812) * t276 + f64x8::splat(1302.8034269168932) * t278 + f64x8::splat(3299.1063064208956) * t280 + f64x8::splat(8289.82636579106) * t282;
            let t285 = t91 * t284;
            let t292 = f64x8::splat(0.4791793102397135) * t271 + f64x8::splat(124.78453667714848) * t276 + f64x8::splat(1389.4444928381436) * t278 + f64x8::splat(3140.9441400416117) * t280 + f64x8::splat(8289.814841116366) * t282;
            let t293 = t206 * t292;
            let t296 = ((t61).select(-f64x8::splat(1.525525181200953) * t247 * t156 + t260 * t85 - t78 * t268, -f64x8::splat(0.47944391030820677) * t272 * t190 + t285 * t109 - t102 * t293));
            let t297 = t296 * t113;
            let t298 = t297 * t121;
            let t301 = t296 * t115;
            let t302 = t301 * t117;
            let t304 = t302 - t301 / f64x8::splat(2.0);
            let t305 = t304 * t120;
            let t306 = t114 * t305;
            let t309 = t231 * t296;
            let t310 = t229 * t309;
            let t314 = ((t3).select(f64x8::splat(0.0), -t132 * t298 / f64x8::splat(12.0) - t20 * t306 / f64x8::splat(4.0) + t132 * t310 / f64x8::splat(4.0)));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t314;
            acc_vsigma = tvsigma0;
            let t317 = ((t44).select(t136, t48 / f64x8::splat(6.0)));
            let t319 = f64x8::splat(2.0) / f64x8::splat(3.0) * t135 * t317;
            let t320 = ((t62).select(f64x8::splat(0.0), -t319));
            let t321 = t320 * t154;
            let t325 = t63 * t320;
            let t327 = t69 * t320;
            let t329 = t71 * t320;
            let t331 = t73 * t320;
            let t333 = -f64x8::splat(2.636397787137096) * t320 + f64x8::splat(10.949031992846576) * t325 - f64x8::splat(37.97192438132487) * t327 + f64x8::splat(16.500233890048545) * t329 - f64x8::splat(152.1256697858192) * t331;
            let t334 = t67 * t333;
            let t341 = -f64x8::splat(1.779981349455627) * t320 + f64x8::splat(7.68676837246043) * t325 - f64x8::splat(28.77361526415555) * t327 + f64x8::splat(8.692072114366688) * t329 - f64x8::splat(152.1256692580183) * t331;
            let t342 = t172 * t341;
            let t345 = ((t87).select(-t319, f64x8::splat(0.0)));
            let t346 = t345 * t183;
            let t350 = t88 * t345;
            let t352 = t93 * t345;
            let t354 = t95 * t345;
            let t356 = t97 * t345;
            let t358 = f64x8::splat(0.5812865360445791) * t345 + f64x8::splat(133.4855290318812) * t350 + f64x8::splat(1302.8034269168932) * t352 + f64x8::splat(3299.1063064208956) * t354 + f64x8::splat(8289.82636579106) * t356;
            let t359 = t91 * t358;
            let t366 = f64x8::splat(0.4791793102397135) * t345 + f64x8::splat(124.78453667714848) * t350 + f64x8::splat(1389.4444928381436) * t352 + f64x8::splat(3140.9441400416117) * t354 + f64x8::splat(8289.814841116366) * t356;
            let t367 = t206 * t366;
            let t370 = ((t61).select(-f64x8::splat(1.525525181200953) * t321 * t156 + t334 * t85 - t78 * t342, -f64x8::splat(0.47944391030820677) * t346 * t190 + t359 * t109 - t102 * t367));
            let t371 = t370 * t113;
            let t372 = t371 * t121;
            let t375 = t370 * t115;
            let t376 = t375 * t117;
            let t378 = t376 - t375 / f64x8::splat(2.0);
            let t379 = t378 * t120;
            let t380 = t114 * t379;
            let t383 = t231 * t370;
            let t384 = t229 * t383;
            let t388 = ((t3).select(f64x8::splat(0.0), -t132 * t372 / f64x8::splat(12.0) - t20 * t380 / f64x8::splat(4.0) + t132 * t384 / f64x8::splat(4.0)));
            let tvlapl0 = f64x8::splat(2.0) * v_rho * t388;
            acc_vlapl = tvlapl0;
            let t392 = ((t44).select(t136, -f64x8::splat(2.0) / f64x8::splat(3.0) * t240 * t28));
            let t394 = f64x8::splat(2.0) / f64x8::splat(3.0) * t135 * t392;
            let t395 = ((t62).select(f64x8::splat(0.0), -t394));
            let t396 = t395 * t154;
            let t400 = t63 * t395;
            let t402 = t69 * t395;
            let t404 = t71 * t395;
            let t406 = t73 * t395;
            let t408 = -f64x8::splat(2.636397787137096) * t395 + f64x8::splat(10.949031992846576) * t400 - f64x8::splat(37.97192438132487) * t402 + f64x8::splat(16.500233890048545) * t404 - f64x8::splat(152.1256697858192) * t406;
            let t409 = t67 * t408;
            let t416 = -f64x8::splat(1.779981349455627) * t395 + f64x8::splat(7.68676837246043) * t400 - f64x8::splat(28.77361526415555) * t402 + f64x8::splat(8.692072114366688) * t404 - f64x8::splat(152.1256692580183) * t406;
            let t417 = t172 * t416;
            let t420 = ((t87).select(-t394, f64x8::splat(0.0)));
            let t421 = t420 * t183;
            let t425 = t88 * t420;
            let t427 = t93 * t420;
            let t429 = t95 * t420;
            let t431 = t97 * t420;
            let t433 = f64x8::splat(0.5812865360445791) * t420 + f64x8::splat(133.4855290318812) * t425 + f64x8::splat(1302.8034269168932) * t427 + f64x8::splat(3299.1063064208956) * t429 + f64x8::splat(8289.82636579106) * t431;
            let t434 = t91 * t433;
            let t441 = f64x8::splat(0.4791793102397135) * t420 + f64x8::splat(124.78453667714848) * t425 + f64x8::splat(1389.4444928381436) * t427 + f64x8::splat(3140.9441400416117) * t429 + f64x8::splat(8289.814841116366) * t431;
            let t442 = t206 * t441;
            let t445 = ((t61).select(-f64x8::splat(1.525525181200953) * t396 * t156 + t409 * t85 - t78 * t417, -f64x8::splat(0.47944391030820677) * t421 * t190 + t434 * t109 - t102 * t442));
            let t446 = t445 * t113;
            let t447 = t446 * t121;
            let t450 = t445 * t115;
            let t453 = t450 * t117 - t450 / f64x8::splat(2.0);
            let t454 = t453 * t120;
            let t455 = t114 * t454;
            let t458 = t231 * t445;
            let t459 = t229 * t458;
            let t463 = ((t3).select(f64x8::splat(0.0), -t132 * t447 / f64x8::splat(12.0) - t20 * t455 / f64x8::splat(4.0) + t132 * t459 / f64x8::splat(4.0)));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t463;
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
