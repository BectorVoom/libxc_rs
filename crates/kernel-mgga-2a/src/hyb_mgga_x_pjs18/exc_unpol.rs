//! HYB_MGGA_X_PJS18 exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_x_pjs18.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn hyb_mgga_x_pjs18_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t13 = t12 <= zeta_threshold;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t13, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = pow_1_3(9.0);
        let t22 = t21 * t21;
        let t24 = pow_1_3(1.0 / M_PI);
        let t25 = t24 * t24;
        let t26 = t22 * t25;
        let t27 = t26 * param_hyb_omega_0;
        let t28 = 1.0 / t19;
        let t30 = M_CBRT6;
        let t31 = M_PI * M_PI;
        let t32 = pow_1_3(t31);
        let t33 = t32 * t32;
        let t34 = 1.0 / t33;
        let t35 = t30 * t34;
        let t36 = M_CBRT2;
        let t37 = t36 * t36;
        let t38 = sigma[ip] * t37;
        let t39 = rho[ip] * rho[ip];
        let t40 = t19 * t19;
        let t42 = 1.0 / t40 / t39;
        let t43 = t38 * t42;
        let t46 = t30 * t30;
        let t48 = 1.0 / t32 / t31;
        let t49 = t46 * t48;
        let t50 = sigma[ip] * sigma[ip];
        let t51 = t50 * t36;
        let t52 = t39 * t39;
        let t53 = t52 * rho[ip];
        let t55 = 1.0 / t19 / t53;
        let t59 = 1.0 + 0.15045488888888888889e0 * t35 * t43 + 0.53798980924525896e-2 * t49 * t51 * t55;
        let t60 = f64::powf(t59, 1.0 / 10.0);
        let t62 = piecewise3(t13, t14, t16);
        let t63 = 1.0 / t62;
        let t64 = 1.0 / t60 * t63;
        let t67 = t27 * t4 * t28 * t64 / 18.0;
        let t68 = t67 < 0.1e-9;
        let t69 = piecewise3(t68, 0.1e-9, t67);
        let t70 = 0.135e1 <= t69;
        let t71 = 0.135e1 < t69;
        let t72 = piecewise3(t71, t69, 0.135e1);
        let t73 = t72 * t72;
        let t76 = t73 * t73;
        let t77 = 1.0 / t76;
        let t79 = t76 * t73;
        let t80 = 1.0 / t79;
        let t82 = t76 * t76;
        let t83 = 1.0 / t82;
        let t86 = 1.0 / t82 / t73;
        let t89 = 1.0 / t82 / t76;
        let t92 = 1.0 / t82 / t79;
        let t94 = t82 * t82;
        let t95 = 1.0 / t94;
        let t98 = piecewise3(t71, 0.135e1, t69);
        let t99 = f64::sqrt(M_PI);
        let t100 = 1.0 / t98;
        let t102 = erf_approx(t100 / 2.0);
        let t104 = t98 * t98;
        let t105 = 1.0 / t104;
        let t107 = f64::exp(-t105 / 4.0);
        let t108 = t107 - 1.0;
        let t111 = t107 - 3.0 / 2.0 - 2.0 * t104 * t108;
        let t114 = t102 * t99 + 2.0 * t111 * t98;
        let t118 = piecewise3(t70, 1.0 / t73 / 36.0 - t77 / 960.0 + t80 / 26880.0 - t83 / 829440.0 + t86 / 28385280.0 - t89 / 0.107347968e10 + t92 / 0.445906944e11 - t95 / 0.20214448128e13, 1.0 - 8.0 / 3.0 * t98 * t114);
        let t119 = f64::powf(t59, 1.0 / 5.0);
        let t120 = 1.0 / t119;
        let t122 = 0.27e0 <= t69;
        let t123 = 0.27e0 < t69;
        let t124 = piecewise3(t123, t69, 0.27e0);
        let t125 = t124 * t124;
        let t126 = t125 * t125;
        let t127 = t126 * t126;
        let t128 = t127 * t126;
        let t129 = t127 * t127;
        let t130 = t129 * t129;
        let t132 = 1.0 / t130 / t128;
        let t134 = t126 * t125;
        let t135 = t127 * t134;
        let t137 = 1.0 / t130 / t135;
        let t141 = 1.0 / t134;
        let t143 = 1.0 / t127;
        let t145 = t127 * t125;
        let t146 = 1.0 / t145;
        let t148 = 1.0 / t128;
        let t150 = 1.0 / t135;
        let t152 = 1.0 / t129;
        let t154 = t129 * t125;
        let t155 = 1.0 / t154;
        let t158 = 1.0 / t129 / t126;
        let t160 = t132 / 0.33929038000650146833571361325056e38 - t137 / 0.3511556992918352140755776405766144e40 + 3.0 / 2240.0 / t126 - t141 / 11520.0 + 3.0 / 788480.0 * t143 - t146 / 7454720.0 + t148 / 0.24772608e9 - t150 / 0.93585408e10 + t152 / 0.3944742912e12 - t155 / 0.183119118336e14 + t158 / 0.9270284255232e15;
        let t162 = 1.0 / t129 / t134;
        let t165 = 1.0 / t129 / t127;
        let t168 = 1.0 / t129 / t145;
        let t171 = 1.0 / t129 / t128;
        let t174 = 1.0 / t129 / t135;
        let t176 = 1.0 / t130;
        let t179 = 1.0 / t130 / t125;
        let t182 = 1.0 / t130 / t126;
        let t185 = 1.0 / t130 / t134;
        let t188 = 1.0 / t130 / t127;
        let t191 = 1.0 / t130 / t145;
        let t193 = -t162 / 0.50785035485184e17 + t165 / 0.2991700272218112e19 - t168 / 0.188514051721003008e21 + t171 / 0.12648942844388573184e23 - t174 / 0.900231674141645733888e24 + t176 / 0.67726520292999771979776e26 - t179 / 0.536974553751641049268224e28 + t182 / 0.44747310348880790522167296e30 - t185 / 0.3909716563474290836848508928e32 + t188 / 0.357385233699457383710280646656e34 - t191 / 0.34109511607036583784813762183168e36;
        let t195 = piecewise3(t123, 0.27e0, t69);
        let t196 = t195 * t195;
        let t198 = t196 * t196;
        let t199 = 64.0 * t198;
        let t200 = 20.0 * t196 - t199;
        let t203 = f64::exp(-1.0 / t196 / 4.0);
        let t207 = 1.0 / t195;
        let t209 = erf_approx(t207 / 2.0);
        let t212 = 10.0 * t195 * t209 * t99 + t200 * t203 - 36.0 * t196 + t199 - 3.0;
        let t216 = piecewise3(t122, t160 + t193, 24.0 * t196 * t212 + 1.0);
        let t217 = tau[ip] * t37;
        let t219 = 1.0 / t40 / rho[ip];
        let t225 = -0.14554132e0 * t217 * t219 + 0.43662396e-1 * t46 * t33 + 0.42296278333333333333e-1 * t43;
        let t226 = t216 * t225;
        let t227 = t119 * t119;
        let t228 = 1.0 / t227;
        let t229 = t35 * t228;
        let t232 = 0.32e0 <= t69;
        let t233 = 0.32e0 < t69;
        let t234 = piecewise3(t233, t69, 0.32e0);
        let t235 = t234 * t234;
        let t236 = t235 * t235;
        let t239 = t236 * t235;
        let t240 = 1.0 / t239;
        let t242 = t236 * t236;
        let t243 = 1.0 / t242;
        let t245 = t242 * t235;
        let t246 = 1.0 / t245;
        let t248 = t242 * t236;
        let t249 = 1.0 / t248;
        let t251 = t242 * t239;
        let t252 = 1.0 / t251;
        let t254 = t242 * t242;
        let t255 = 1.0 / t254;
        let t258 = 1.0 / t254 / t235;
        let t261 = 1.0 / t254 / t236;
        let t264 = 1.0 / t254 / t239;
        let t267 = 1.0 / t254 / t242;
        let t270 = 1.0 / t254 / t245;
        let t273 = 1.0 / t254 / t248;
        let t276 = 1.0 / t254 / t251;
        let t278 = t254 * t254;
        let t279 = 1.0 / t278;
        let t282 = 1.0 / t278 / t235;
        let t285 = 1.0 / t278 / t236;
        let t288 = 1.0 / t278 / t239;
        let t290 = 3.0 / 7840.0 / t236 - t240 / 56448.0 + 5.0 / 8515584.0 * t243 - t246 / 61501440.0 + t249 / 0.253034496e10 - t252 / 0.1158119424e12 + t255 / 0.581192122368e13 - t258 / 0.316612955602944e15 + t261 / 0.185827061661696e17 - t264 / 0.1168055816159232e19 + t267 / 0.7824446865801216e20 - t270 / 0.55625110547104530432e22 + t273 / 0.41817405043548622946304e24 - t276 / 0.33139778504339333578752e26 + t279 / 0.2760851680179343645999104e28 - t282 / 0.24119107039344543796297728e30 + t285 / 0.22046293272414372635684634624e32 - t288 / 0.21042094544618633283918675050496e34;
        let t291 = piecewise3(t233, 0.32e0, t69);
        let t293 = t291 * t291;
        let t294 = t293 * t291;
        let t296 = t293 * t293;
        let t297 = t296 * t291;
        let t299 = t296 * t294;
        let t301 = t296 * t296;
        let t302 = t301 * t291;
        let t304 = -8.0 * t291 + 256.0 * t294 - 576.0 * t297 + 3840.0 * t299 - 122880.0 * t302;
        let t305 = 1.0 / t293;
        let t307 = f64::exp(-t305 / 4.0);
        let t311 = t296 * t293;
        let t313 = -35.0 + 224.0 * t293 - 1440.0 * t296 + 5120.0 * t311;
        let t317 = -2.0 + 60.0 * t293;
        let t319 = 1.0 / t291;
        let t321 = erf_approx(t319 / 2.0);
        let t324 = 2.0 * t317 * t321 * t99 + 24.0 * t294 * t313 + t304 * t307;
        let t328 = piecewise3(t232, t290, 1.0 + 8.0 / 7.0 * t291 * t324);
        let t329 = t328 * t30;
        let t330 = t329 * t34;
        let t332 = t38 * t42 * t228;
        let t335 = t118 * t120 + 35.0 / 81.0 * t226 * t229 + 0.26329605555555555556e-1 * t330 * t332;
        let t339 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t335);
        let tzk0 = 2.0 * t339;
        zk[ip] += tzk0;
    }
}
