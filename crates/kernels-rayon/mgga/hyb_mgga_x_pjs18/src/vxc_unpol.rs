//! HYB_MGGA_X_PJS18 vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_x_pjs18.c`
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
pub fn hyb_mgga_x_pjs18_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
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
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 / t5;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t13 = (t12).simd_le(zeta_threshold);
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = ((t13).select(t14 * zeta_threshold, t16 * t12));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = (simd::cbrt(f64x8::splat(9.0)));
            let t22 = t21 * t21;
            let t24 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t25 = t24 * t24;
            let t26 = t22 * t25;
            let t27 = t26 * param_hyb_omega_0;
            let t28 = f64x8::splat(1.0) / t19;
            let t30 = f64x8::splat(M_CBRT6);
            let t31 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t32 = (simd::cbrt(t31));
            let t33 = t32 * t32;
            let t34 = f64x8::splat(1.0) / t33;
            let t35 = t30 * t34;
            let t36 = f64x8::splat(M_CBRT2);
            let t37 = t36 * t36;
            let t38 = v_sigma * t37;
            let t39 = v_rho * v_rho;
            let t40 = t19 * t19;
            let t42 = f64x8::splat(1.0) / t40 / t39;
            let t43 = t38 * t42;
            let t46 = t30 * t30;
            let t48 = f64x8::splat(1.0) / t32 / t31;
            let t49 = t46 * t48;
            let t50 = v_sigma * v_sigma;
            let t51 = t50 * t36;
            let t52 = t39 * t39;
            let t53 = t52 * v_rho;
            let t55 = f64x8::splat(1.0) / t19 / t53;
            let t59 = f64x8::splat(1.0) + f64x8::splat(0.1504548888888889) * t35 * t43 + f64x8::splat(0.00537989809245259) * t49 * t51 * t55;
            let t60 = (simd::pow(t59, f64x8::splat(1.0) / f64x8::splat(10.0)));
            let t62 = ((t13).select(t14, t16));
            let t63 = f64x8::splat(1.0) / t62;
            let t64 = f64x8::splat(1.0) / t60 * t63;
            let t67 = t27 * t4 * t28 * t64 / f64x8::splat(18.0);
            let t68 = (t67).simd_lt(f64x8::splat(1e-10));
            let t69 = ((t68).select(f64x8::splat(1e-10), t67));
            let t70 = (f64x8::splat(1.35)).simd_le(t69);
            let t71 = (f64x8::splat(1.35)).simd_lt(t69);
            let t72 = ((t71).select(t69, f64x8::splat(1.35)));
            let t73 = t72 * t72;
            let t76 = t73 * t73;
            let t77 = f64x8::splat(1.0) / t76;
            let t79 = t76 * t73;
            let t80 = f64x8::splat(1.0) / t79;
            let t82 = t76 * t76;
            let t83 = f64x8::splat(1.0) / t82;
            let t86 = f64x8::splat(1.0) / t82 / t73;
            let t89 = f64x8::splat(1.0) / t82 / t76;
            let t92 = f64x8::splat(1.0) / t82 / t79;
            let t94 = t82 * t82;
            let t95 = f64x8::splat(1.0) / t94;
            let t98 = ((t71).select(f64x8::splat(1.35), t69));
            let t99 = ((f64x8::splat(M_PI)).sqrt());
            let t100 = f64x8::splat(1.0) / t98;
            let t102 = (simd::erf(t100 / f64x8::splat(2.0)));
            let t104 = t98 * t98;
            let t105 = f64x8::splat(1.0) / t104;
            let t107 = (simd::exp(-t105 / f64x8::splat(4.0)));
            let t108 = t107 - f64x8::splat(1.0);
            let t111 = t107 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t104 * t108;
            let t114 = t102 * t99 + f64x8::splat(2.0) * t111 * t98;
            let t118 = ((t70).select(f64x8::splat(1.0) / t73 / f64x8::splat(36.0) - t77 / f64x8::splat(960.0) + t80 / f64x8::splat(26880.0) - t83 / f64x8::splat(829440.0) + t86 / f64x8::splat(28385280.0) - t89 / f64x8::splat(1073479680.0) + t92 / f64x8::splat(44590694400.0) - t95 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t98 * t114));
            let t119 = (simd::pow(t59, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t120 = f64x8::splat(1.0) / t119;
            let t122 = (f64x8::splat(0.27)).simd_le(t69);
            let t123 = (f64x8::splat(0.27)).simd_lt(t69);
            let t124 = ((t123).select(t69, f64x8::splat(0.27)));
            let t125 = t124 * t124;
            let t126 = t125 * t125;
            let t127 = t126 * t126;
            let t128 = t127 * t126;
            let t129 = t127 * t127;
            let t130 = t129 * t129;
            let t132 = f64x8::splat(1.0) / t130 / t128;
            let t134 = t126 * t125;
            let t135 = t127 * t134;
            let t137 = f64x8::splat(1.0) / t130 / t135;
            let t141 = f64x8::splat(1.0) / t134;
            let t143 = f64x8::splat(1.0) / t127;
            let t145 = t127 * t125;
            let t146 = f64x8::splat(1.0) / t145;
            let t148 = f64x8::splat(1.0) / t128;
            let t150 = f64x8::splat(1.0) / t135;
            let t152 = f64x8::splat(1.0) / t129;
            let t154 = t129 * t125;
            let t155 = f64x8::splat(1.0) / t154;
            let t158 = f64x8::splat(1.0) / t129 / t126;
            let t160 = t132 / f64x8::splat(3.3929038000650147e+37) - t137 / f64x8::splat(3.511556992918352e+39) + f64x8::splat(3.0) / f64x8::splat(2240.0) / t126 - t141 / f64x8::splat(11520.0) + f64x8::splat(3.0) / f64x8::splat(788480.0) * t143 - t146 / f64x8::splat(7454720.0) + t148 / f64x8::splat(247726080.0) - t150 / f64x8::splat(9358540800.0) + t152 / f64x8::splat(394474291200.0) - t155 / f64x8::splat(18311911833600.0) + t158 / f64x8::splat(927028425523200.0);
            let t162 = f64x8::splat(1.0) / t129 / t134;
            let t165 = f64x8::splat(1.0) / t129 / t127;
            let t168 = f64x8::splat(1.0) / t129 / t145;
            let t171 = f64x8::splat(1.0) / t129 / t128;
            let t174 = f64x8::splat(1.0) / t129 / t135;
            let t176 = f64x8::splat(1.0) / t130;
            let t179 = f64x8::splat(1.0) / t130 / t125;
            let t182 = f64x8::splat(1.0) / t130 / t126;
            let t185 = f64x8::splat(1.0) / t130 / t134;
            let t188 = f64x8::splat(1.0) / t130 / t127;
            let t191 = f64x8::splat(1.0) / t130 / t145;
            let t193 = -t162 / f64x8::splat(5.0785035485184e+16) + t165 / f64x8::splat(2.991700272218112e+18) - t168 / f64x8::splat(1.88514051721003e+20) + t171 / f64x8::splat(1.2648942844388573e+22) - t174 / f64x8::splat(9.002316741416457e+23) + t176 / f64x8::splat(6.772652029299977e+25) - t179 / f64x8::splat(5.36974553751641e+27) + t182 / f64x8::splat(4.474731034888079e+29) - t185 / f64x8::splat(3.909716563474291e+31) + t188 / f64x8::splat(3.5738523369945735e+33) - t191 / f64x8::splat(3.410951160703658e+35);
            let t195 = ((t123).select(f64x8::splat(0.27), t69));
            let t196 = t195 * t195;
            let t198 = t196 * t196;
            let t199 = f64x8::splat(64.0) * t198;
            let t200 = f64x8::splat(20.0) * t196 - t199;
            let t203 = (simd::exp(-f64x8::splat(1.0) / t196 / f64x8::splat(4.0)));
            let t207 = f64x8::splat(1.0) / t195;
            let t209 = (simd::erf(t207 / f64x8::splat(2.0)));
            let t212 = f64x8::splat(10.0) * t195 * t209 * t99 + t200 * t203 - f64x8::splat(36.0) * t196 + t199 - f64x8::splat(3.0);
            let t216 = ((t122).select(t160 + t193, f64x8::splat(24.0) * t196 * t212 + f64x8::splat(1.0)));
            let t217 = v_tau * t37;
            let t219 = f64x8::splat(1.0) / t40 / v_rho;
            let t225 = -f64x8::splat(0.14554132) * t217 * t219 + f64x8::splat(0.043662396) * t46 * t33 + f64x8::splat(0.04229627833333333) * t43;
            let t226 = t216 * t225;
            let t227 = t119 * t119;
            let t228 = f64x8::splat(1.0) / t227;
            let t229 = t35 * t228;
            let t232 = (f64x8::splat(0.32)).simd_le(t69);
            let t233 = (f64x8::splat(0.32)).simd_lt(t69);
            let t234 = ((t233).select(t69, f64x8::splat(0.32)));
            let t235 = t234 * t234;
            let t236 = t235 * t235;
            let t239 = t236 * t235;
            let t240 = f64x8::splat(1.0) / t239;
            let t242 = t236 * t236;
            let t243 = f64x8::splat(1.0) / t242;
            let t245 = t242 * t235;
            let t246 = f64x8::splat(1.0) / t245;
            let t248 = t242 * t236;
            let t249 = f64x8::splat(1.0) / t248;
            let t251 = t242 * t239;
            let t252 = f64x8::splat(1.0) / t251;
            let t254 = t242 * t242;
            let t255 = f64x8::splat(1.0) / t254;
            let t258 = f64x8::splat(1.0) / t254 / t235;
            let t261 = f64x8::splat(1.0) / t254 / t236;
            let t264 = f64x8::splat(1.0) / t254 / t239;
            let t267 = f64x8::splat(1.0) / t254 / t242;
            let t270 = f64x8::splat(1.0) / t254 / t245;
            let t273 = f64x8::splat(1.0) / t254 / t248;
            let t276 = f64x8::splat(1.0) / t254 / t251;
            let t278 = t254 * t254;
            let t279 = f64x8::splat(1.0) / t278;
            let t282 = f64x8::splat(1.0) / t278 / t235;
            let t285 = f64x8::splat(1.0) / t278 / t236;
            let t288 = f64x8::splat(1.0) / t278 / t239;
            let t290 = f64x8::splat(3.0) / f64x8::splat(7840.0) / t236 - t240 / f64x8::splat(56448.0) + f64x8::splat(5.0) / f64x8::splat(8515584.0) * t243 - t246 / f64x8::splat(61501440.0) + t249 / f64x8::splat(2530344960.0) - t252 / f64x8::splat(115811942400.0) + t255 / f64x8::splat(5811921223680.0) - t258 / f64x8::splat(316612955602944.0) + t261 / f64x8::splat(1.85827061661696e+16) - t264 / f64x8::splat(1.168055816159232e+18) + t267 / f64x8::splat(7.824446865801216e+19) - t270 / f64x8::splat(5.562511054710453e+21) + t273 / f64x8::splat(4.181740504354862e+23) - t276 / f64x8::splat(3.3139778504339334e+25) + t279 / f64x8::splat(2.7608516801793436e+27) - t282 / f64x8::splat(2.4119107039344544e+29) + t285 / f64x8::splat(2.2046293272414373e+31) - t288 / f64x8::splat(2.1042094544618633e+33);
            let t291 = ((t233).select(f64x8::splat(0.32), t69));
            let t293 = t291 * t291;
            let t294 = t293 * t291;
            let t296 = t293 * t293;
            let t297 = t296 * t291;
            let t299 = t296 * t294;
            let t301 = t296 * t296;
            let t302 = t301 * t291;
            let t304 = -f64x8::splat(8.0) * t291 + f64x8::splat(256.0) * t294 - f64x8::splat(576.0) * t297 + f64x8::splat(3840.0) * t299 - f64x8::splat(122880.0) * t302;
            let t305 = f64x8::splat(1.0) / t293;
            let t307 = (simd::exp(-t305 / f64x8::splat(4.0)));
            let t311 = t296 * t293;
            let t313 = -f64x8::splat(35.0) + f64x8::splat(224.0) * t293 - f64x8::splat(1440.0) * t296 + f64x8::splat(5120.0) * t311;
            let t317 = -f64x8::splat(2.0) + f64x8::splat(60.0) * t293;
            let t319 = f64x8::splat(1.0) / t291;
            let t321 = (simd::erf(t319 / f64x8::splat(2.0)));
            let t324 = f64x8::splat(2.0) * t317 * t321 * t99 + f64x8::splat(24.0) * t294 * t313 + t304 * t307;
            let t328 = ((t232).select(t290, f64x8::splat(1.0) + f64x8::splat(8.0) / f64x8::splat(7.0) * t291 * t324));
            let t329 = t328 * t30;
            let t330 = t329 * t34;
            let t332 = t38 * t42 * t228;
            let t335 = t118 * t120 + f64x8::splat(35.0) / f64x8::splat(81.0) * t226 * t229 + f64x8::splat(0.026329605555555555) * t330 * t332;
            let t339 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t335));
            let tzk0 = f64x8::splat(2.0) * t339;
            acc_zk = tzk0;
            let t341 = t18 / t40;
            let t345 = t73 * t72;
            let t346 = f64x8::splat(1.0) / t345;
            let t348 = f64x8::splat(1.0) / t19 / v_rho;
            let t353 = param_hyb_omega_0 * t4;
            let t354 = t26 * t353;
            let t356 = f64x8::splat(1.0) / t60 / t59;
            let t357 = t28 * t356;
            let t358 = t39 * v_rho;
            let t360 = f64x8::splat(1.0) / t40 / t358;
            let t361 = t38 * t360;
            let t364 = t52 * t39;
            let t366 = f64x8::splat(1.0) / t19 / t364;
            let t370 = -f64x8::splat(0.40121303703703703) * t35 * t361 - f64x8::splat(0.028692789826413812) * t49 * t51 * t366;
            let t371 = t63 * t370;
            let t376 = ((t68).select(f64x8::splat(0.0), -t27 * t4 * t348 * t64 / f64x8::splat(54.0) - t354 * t357 * t371 / f64x8::splat(180.0)));
            let t377 = ((t71).select(t376, f64x8::splat(0.0)));
            let t380 = t76 * t72;
            let t381 = f64x8::splat(1.0) / t380;
            let t384 = t76 * t345;
            let t385 = f64x8::splat(1.0) / t384;
            let t389 = f64x8::splat(1.0) / t82 / t72;
            let t393 = f64x8::splat(1.0) / t82 / t345;
            let t397 = f64x8::splat(1.0) / t82 / t380;
            let t401 = f64x8::splat(1.0) / t82 / t384;
            let t405 = f64x8::splat(1.0) / t94 / t72;
            let t409 = ((t71).select(f64x8::splat(0.0), t376));
            let t411 = t107 * t105;
            let t415 = t104 * t98;
            let t416 = f64x8::splat(1.0) / t415;
            let t420 = t98 * t108;
            let t425 = t416 * t409 * t107 / f64x8::splat(2.0) - f64x8::splat(4.0) * t420 * t409 - t100 * t409 * t107;
            let t428 = f64x8::splat(2.0) * t111 * t409 - t409 * t411 + f64x8::splat(2.0) * t425 * t98;
            let t432 = ((t70).select(-t346 * t377 / f64x8::splat(18.0) + t381 * t377 / f64x8::splat(240.0) - t385 * t377 / f64x8::splat(4480.0) + t389 * t377 / f64x8::splat(103680.0) - t393 * t377 / f64x8::splat(2838528.0) + t397 * t377 / f64x8::splat(89456640.0) - t401 * t377 / f64x8::splat(3185049600.0) + t405 * t377 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t409 * t114 - f64x8::splat(8.0) / f64x8::splat(3.0) * t98 * t428));
            let t435 = f64x8::splat(1.0) / t119 / t59;
            let t436 = t118 * t435;
            let t439 = t126 * t124;
            let t440 = t127 * t439;
            let t442 = f64x8::splat(1.0) / t130 / t440;
            let t443 = ((t123).select(t376, f64x8::splat(0.0)));
            let t446 = t125 * t124;
            let t447 = t126 * t446;
            let t448 = t127 * t447;
            let t450 = f64x8::splat(1.0) / t130 / t448;
            let t453 = f64x8::splat(1.0) / t439;
            let t456 = f64x8::splat(1.0) / t447;
            let t459 = t127 * t124;
            let t460 = f64x8::splat(1.0) / t459;
            let t463 = t127 * t446;
            let t464 = f64x8::splat(1.0) / t463;
            let t467 = f64x8::splat(1.0) / t440;
            let t470 = f64x8::splat(1.0) / t448;
            let t473 = t129 * t124;
            let t474 = f64x8::splat(1.0) / t473;
            let t478 = f64x8::splat(1.0) / t129 / t446;
            let t482 = f64x8::splat(1.0) / t129 / t439;
            let t485 = -t442 * t443 / f64x8::splat(7.71114500014776e+35) + t450 * t443 / f64x8::splat(7.633819549822504e+37) - f64x8::splat(3.0) / f64x8::splat(560.0) * t453 * t443 + t456 * t443 / f64x8::splat(1920.0) - f64x8::splat(3.0) / f64x8::splat(98560.0) * t460 * t443 + t464 * t443 / f64x8::splat(745472.0) - t467 * t443 / f64x8::splat(20643840.0) + t470 * t443 / f64x8::splat(668467200.0) - t474 * t443 / f64x8::splat(24654643200.0) + t478 * t443 / f64x8::splat(1017328435200.0) - t482 * t443 / f64x8::splat(46351421276160.0);
            let t487 = f64x8::splat(1.0) / t129 / t447;
            let t491 = f64x8::splat(1.0) / t129 / t459;
            let t495 = f64x8::splat(1.0) / t129 / t463;
            let t499 = f64x8::splat(1.0) / t129 / t440;
            let t503 = f64x8::splat(1.0) / t129 / t448;
            let t507 = f64x8::splat(1.0) / t130 / t124;
            let t511 = f64x8::splat(1.0) / t130 / t446;
            let t515 = f64x8::splat(1.0) / t130 / t439;
            let t519 = f64x8::splat(1.0) / t130 / t447;
            let t523 = f64x8::splat(1.0) / t130 / t459;
            let t527 = f64x8::splat(1.0) / t130 / t463;
            let t530 = t487 * t443 / f64x8::splat(2308410703872000.0) - t491 * t443 / f64x8::splat(1.24654178009088e+17) + t495 * t443 / f64x8::splat(7.250540450807808e+18) - t499 * t443 / f64x8::splat(4.517479587281633e+20) + t503 * t443 / f64x8::splat(3.000772247138819e+22) - t507 * t443 / f64x8::splat(2.116453759156243e+24) + t511 * t443 / f64x8::splat(1.5793369227989443e+26) - t515 * t443 / f64x8::splat(1.2429808430244664e+28) + t519 * t443 / f64x8::splat(1.0288727798616555e+30) - t523 * t443 / f64x8::splat(8.934630842486435e+31) + t527 * t443 / f64x8::splat(8.121312287389663e+33);
            let t532 = t195 * t212;
            let t533 = ((t123).select(f64x8::splat(0.0), t376));
            let t536 = t195 * t533;
            let t538 = t196 * t195;
            let t540 = f64x8::splat(256.0) * t538 * t533;
            let t541 = f64x8::splat(40.0) * t536 - t540;
            let t543 = f64x8::splat(1.0) / t538;
            let t544 = t200 * t543;
            let t545 = t533 * t203;
            let t552 = t207 * t203;
            let t555 = t541 * t203 + t544 * t545 / f64x8::splat(2.0) - f64x8::splat(72.0) * t536 + t540 + f64x8::splat(10.0) * t533 * t99 * t209 - f64x8::splat(10.0) * t552 * t533;
            let t559 = ((t122).select(t485 + t530, f64x8::splat(24.0) * t196 * t555 + f64x8::splat(48.0) * t532 * t533));
            let t560 = t559 * t225;
            let t566 = f64x8::splat(0.24256886666666666) * t217 * t42 - f64x8::splat(0.11279007555555555) * t361;
            let t567 = t216 * t566;
            let t570 = t226 * t30;
            let t572 = f64x8::splat(1.0) / t227 / t59;
            let t573 = t34 * t572;
            let t574 = t573 * t370;
            let t577 = t236 * t234;
            let t578 = f64x8::splat(1.0) / t577;
            let t579 = ((t233).select(t376, f64x8::splat(0.0)));
            let t582 = t235 * t234;
            let t583 = t236 * t582;
            let t584 = f64x8::splat(1.0) / t583;
            let t587 = t242 * t234;
            let t588 = f64x8::splat(1.0) / t587;
            let t591 = t242 * t582;
            let t592 = f64x8::splat(1.0) / t591;
            let t595 = t242 * t577;
            let t596 = f64x8::splat(1.0) / t595;
            let t599 = t242 * t583;
            let t600 = f64x8::splat(1.0) / t599;
            let t604 = f64x8::splat(1.0) / t254 / t234;
            let t608 = f64x8::splat(1.0) / t254 / t582;
            let t612 = f64x8::splat(1.0) / t254 / t577;
            let t616 = f64x8::splat(1.0) / t254 / t583;
            let t620 = f64x8::splat(1.0) / t254 / t587;
            let t624 = f64x8::splat(1.0) / t254 / t591;
            let t628 = f64x8::splat(1.0) / t254 / t595;
            let t632 = f64x8::splat(1.0) / t254 / t599;
            let t636 = f64x8::splat(1.0) / t278 / t234;
            let t640 = f64x8::splat(1.0) / t278 / t582;
            let t644 = f64x8::splat(1.0) / t278 / t577;
            let t648 = f64x8::splat(1.0) / t278 / t583;
            let t651 = -f64x8::splat(3.0) / f64x8::splat(1960.0) * t578 * t579 + t584 * t579 / f64x8::splat(9408.0) - f64x8::splat(5.0) / f64x8::splat(1064448.0) * t588 * t579 + t592 * t579 / f64x8::splat(6150144.0) - t596 * t579 / f64x8::splat(210862080.0) + t600 * t579 / f64x8::splat(8272281600.0) - t604 * t579 / f64x8::splat(363245076480.0) + t608 * t579 / f64x8::splat(17589608644608.0) - t612 * t579 / f64x8::splat(929135308308480.0) + t616 * t579 / f64x8::splat(5.3093446189056e+16) - t620 * t579 / f64x8::splat(3.26018619408384e+18) + t624 * t579 / f64x8::splat(2.1394273287347896e+20) - t628 * t579 / f64x8::splat(1.493478751555308e+22) + t632 * t579 / f64x8::splat(1.1046592834779778e+24) - t636 * t579 / f64x8::splat(8.627661500560449e+25) + t640 * t579 / f64x8::splat(7.093855011571925e+27) - t644 * t579 / f64x8::splat(6.123970353448437e+29) + t648 * t579 / f64x8::splat(5.53739330121543e+31);
            let t652 = ((t233).select(f64x8::splat(0.0), t376));
            let t655 = t293 * t652;
            let t657 = t296 * t652;
            let t663 = -f64x8::splat(1105920.0) * t301 * t652 + f64x8::splat(26880.0) * t311 * t652 - f64x8::splat(8.0) * t652 + f64x8::splat(768.0) * t655 - f64x8::splat(2880.0) * t657;
            let t665 = f64x8::splat(1.0) / t294;
            let t666 = t304 * t665;
            let t667 = t652 * t307;
            let t670 = t293 * t313;
            let t673 = t291 * t652;
            let t675 = t294 * t652;
            let t677 = t297 * t652;
            let t679 = f64x8::splat(448.0) * t673 - f64x8::splat(5760.0) * t675 + f64x8::splat(30720.0) * t677;
            let t682 = t99 * t291;
            let t686 = t317 * t307;
            let t687 = t305 * t652;
            let t690 = t663 * t307 + t666 * t667 / f64x8::splat(2.0) + f64x8::splat(72.0) * t670 * t652 + f64x8::splat(24.0) * t294 * t679 + f64x8::splat(240.0) * t682 * t652 * t321 - f64x8::splat(2.0) * t686 * t687;
            let t694 = ((t232).select(t651, f64x8::splat(8.0) / f64x8::splat(7.0) * t291 * t690 + f64x8::splat(8.0) / f64x8::splat(7.0) * t652 * t324));
            let t695 = t694 * t30;
            let t696 = t695 * t34;
            let t700 = t38 * t360 * t228;
            let t703 = t34 * v_sigma;
            let t704 = t329 * t703;
            let t705 = t37 * t42;
            let t706 = t572 * t370;
            let t707 = t705 * t706;
            let t710 = t432 * t120 - t436 * t370 / f64x8::splat(5.0) + f64x8::splat(35.0) / f64x8::splat(81.0) * t560 * t229 + f64x8::splat(35.0) / f64x8::splat(81.0) * t567 * t229 - f64x8::splat(14.0) / f64x8::splat(81.0) * t570 * t574 + f64x8::splat(0.026329605555555555) * t696 * t332 - f64x8::splat(0.07021228148148148) * t330 * t700 - f64x8::splat(0.010531842222222223) * t704 * t707;
            let t715 = ((t3).select(f64x8::splat(0.0), -t7 * t341 * t335 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t710));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t715 + f64x8::splat(2.0) * t339;
            acc_vrho = tvrho0;
            let t720 = v_sigma * t36;
            let t724 = f64x8::splat(0.1504548888888889) * t35 * t705 + f64x8::splat(0.01075979618490518) * t49 * t720 * t55;
            let t725 = t63 * t724;
            let t729 = ((t68).select(f64x8::splat(0.0), -t354 * t357 * t725 / f64x8::splat(180.0)));
            let t730 = ((t71).select(t729, f64x8::splat(0.0)));
            let t733 = t381 * t730;
            let t735 = t385 * t730;
            let t737 = t389 * t730;
            let t739 = t393 * t730;
            let t741 = t397 * t730;
            let t743 = t401 * t730;
            let t745 = t405 * t730;
            let t748 = ((t71).select(f64x8::splat(0.0), t729));
            let t760 = t416 * t748 * t107 / f64x8::splat(2.0) - f64x8::splat(4.0) * t420 * t748 - t100 * t748 * t107;
            let t763 = f64x8::splat(2.0) * t111 * t748 - t411 * t748 + f64x8::splat(2.0) * t760 * t98;
            let t767 = ((t70).select(-t346 * t730 / f64x8::splat(18.0) + t733 / f64x8::splat(240.0) - t735 / f64x8::splat(4480.0) + t737 / f64x8::splat(103680.0) - t739 / f64x8::splat(2838528.0) + t741 / f64x8::splat(89456640.0) - t743 / f64x8::splat(3185049600.0) + t745 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t748 * t114 - f64x8::splat(8.0) / f64x8::splat(3.0) * t98 * t763));
            let t771 = ((t123).select(t729, f64x8::splat(0.0)));
            let t772 = t442 * t771;
            let t774 = t450 * t771;
            let t778 = t456 * t771;
            let t780 = t460 * t771;
            let t782 = t464 * t771;
            let t784 = t467 * t771;
            let t786 = t470 * t771;
            let t788 = t474 * t771;
            let t790 = t478 * t771;
            let t792 = t482 * t771;
            let t794 = -t772 / f64x8::splat(7.71114500014776e+35) + t774 / f64x8::splat(7.633819549822504e+37) - f64x8::splat(3.0) / f64x8::splat(560.0) * t453 * t771 + t778 / f64x8::splat(1920.0) - f64x8::splat(3.0) / f64x8::splat(98560.0) * t780 + t782 / f64x8::splat(745472.0) - t784 / f64x8::splat(20643840.0) + t786 / f64x8::splat(668467200.0) - t788 / f64x8::splat(24654643200.0) + t790 / f64x8::splat(1017328435200.0) - t792 / f64x8::splat(46351421276160.0);
            let t795 = t487 * t771;
            let t797 = t491 * t771;
            let t799 = t495 * t771;
            let t801 = t499 * t771;
            let t803 = t503 * t771;
            let t805 = t507 * t771;
            let t807 = t511 * t771;
            let t809 = t515 * t771;
            let t811 = t519 * t771;
            let t813 = t523 * t771;
            let t815 = t527 * t771;
            let t817 = t795 / f64x8::splat(2308410703872000.0) - t797 / f64x8::splat(1.24654178009088e+17) + t799 / f64x8::splat(7.250540450807808e+18) - t801 / f64x8::splat(4.517479587281633e+20) + t803 / f64x8::splat(3.000772247138819e+22) - t805 / f64x8::splat(2.116453759156243e+24) + t807 / f64x8::splat(1.5793369227989443e+26) - t809 / f64x8::splat(1.2429808430244664e+28) + t811 / f64x8::splat(1.0288727798616555e+30) - t813 / f64x8::splat(8.934630842486435e+31) + t815 / f64x8::splat(8.121312287389663e+33);
            let t819 = ((t123).select(f64x8::splat(0.0), t729));
            let t822 = t195 * t819;
            let t825 = f64x8::splat(256.0) * t538 * t819;
            let t826 = f64x8::splat(40.0) * t822 - t825;
            let t828 = t819 * t203;
            let t837 = t826 * t203 + t544 * t828 / f64x8::splat(2.0) - f64x8::splat(72.0) * t822 + t825 + f64x8::splat(10.0) * t819 * t99 * t209 - f64x8::splat(10.0) * t552 * t819;
            let t841 = ((t122).select(t794 + t817, f64x8::splat(24.0) * t196 * t837 + f64x8::splat(48.0) * t532 * t819));
            let t842 = t841 * t225;
            let t845 = t216 * t37;
            let t846 = t845 * t42;
            let t849 = t573 * t724;
            let t852 = ((t233).select(t729, f64x8::splat(0.0)));
            let t855 = t584 * t852;
            let t857 = t588 * t852;
            let t859 = t592 * t852;
            let t861 = t596 * t852;
            let t863 = t600 * t852;
            let t865 = t604 * t852;
            let t867 = t608 * t852;
            let t869 = t612 * t852;
            let t871 = t616 * t852;
            let t873 = t620 * t852;
            let t875 = t624 * t852;
            let t877 = t628 * t852;
            let t879 = t632 * t852;
            let t881 = t636 * t852;
            let t883 = t640 * t852;
            let t885 = t644 * t852;
            let t887 = t648 * t852;
            let t889 = -f64x8::splat(3.0) / f64x8::splat(1960.0) * t578 * t852 + t855 / f64x8::splat(9408.0) - f64x8::splat(5.0) / f64x8::splat(1064448.0) * t857 + t859 / f64x8::splat(6150144.0) - t861 / f64x8::splat(210862080.0) + t863 / f64x8::splat(8272281600.0) - t865 / f64x8::splat(363245076480.0) + t867 / f64x8::splat(17589608644608.0) - t869 / f64x8::splat(929135308308480.0) + t871 / f64x8::splat(5.3093446189056e+16) - t873 / f64x8::splat(3.26018619408384e+18) + t875 / f64x8::splat(2.1394273287347896e+20) - t877 / f64x8::splat(1.493478751555308e+22) + t879 / f64x8::splat(1.1046592834779778e+24) - t881 / f64x8::splat(8.627661500560449e+25) + t883 / f64x8::splat(7.093855011571925e+27) - t885 / f64x8::splat(6.123970353448437e+29) + t887 / f64x8::splat(5.53739330121543e+31);
            let t890 = ((t233).select(f64x8::splat(0.0), t729));
            let t893 = t293 * t890;
            let t895 = t296 * t890;
            let t897 = t311 * t890;
            let t901 = -f64x8::splat(1105920.0) * t301 * t890 - f64x8::splat(8.0) * t890 + f64x8::splat(768.0) * t893 - f64x8::splat(2880.0) * t895 + f64x8::splat(26880.0) * t897;
            let t903 = t890 * t307;
            let t908 = t291 * t890;
            let t910 = t294 * t890;
            let t912 = t297 * t890;
            let t914 = f64x8::splat(448.0) * t908 - f64x8::splat(5760.0) * t910 + f64x8::splat(30720.0) * t912;
            let t917 = t890 * t321;
            let t923 = t901 * t307 + t666 * t903 / f64x8::splat(2.0) + f64x8::splat(72.0) * t670 * t890 + f64x8::splat(24.0) * t294 * t914 + f64x8::splat(240.0) * t682 * t917 - f64x8::splat(2.0) * t686 * t305 * t890;
            let t927 = ((t232).select(t889, f64x8::splat(8.0) / f64x8::splat(7.0) * t291 * t923 + f64x8::splat(8.0) / f64x8::splat(7.0) * t890 * t324));
            let t928 = t927 * t30;
            let t929 = t928 * t34;
            let t932 = t705 * t228;
            let t935 = t572 * t724;
            let t936 = t705 * t935;
            let t939 = t767 * t120 - t436 * t724 / f64x8::splat(5.0) + f64x8::splat(35.0) / f64x8::splat(81.0) * t842 * t229 + f64x8::splat(0.018276169650205763) * t846 * t229 - f64x8::splat(14.0) / f64x8::splat(81.0) * t570 * t849 + f64x8::splat(0.026329605555555555) * t929 * t332 + f64x8::splat(0.026329605555555555) * t330 * t932 - f64x8::splat(0.010531842222222223) * t704 * t936;
            let t943 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t939));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t943;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t945 = t4 * t18;
            let t946 = t348 * t216;
            let t947 = t945 * t946;
            let t948 = t37 * t30;
            let t950 = t948 * t34 * t228;
            let t953 = ((t3).select(f64x8::splat(0.0), f64x8::splat(0.01610215409462904) * t947 * t950));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t953;
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
