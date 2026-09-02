//! HYB_MGGA_X_PJS18 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_x_pjs18.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn hyb_mgga_x_pjs18_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
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
        let t59 = 1.0 + 0.1504548888888889 * t35 * t43 + 0.00537989809245259 * t49 * t51 * t55;
        let t60 = rmath::pow(t59, 1.0 / 10.0);
        let t62 = piecewise3(t13, t14, t16);
        let t63 = 1.0 / t62;
        let t64 = 1.0 / t60 * t63;
        let t67 = t27 * t4 * t28 * t64 / 18.0;
        let t68 = t67 < 1e-10;
        let t69 = piecewise3(t68, 1e-10, t67);
        let t70 = 1.35 <= t69;
        let t71 = 1.35 < t69;
        let t72 = piecewise3(t71, t69, 1.35);
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
        let t98 = piecewise3(t71, 1.35, t69);
        let t99 = rmath::sqrt(M_PI);
        let t100 = 1.0 / t98;
        let t102 = rmath::erf(t100 / 2.0);
        let t104 = t98 * t98;
        let t105 = 1.0 / t104;
        let t107 = rmath::exp(-t105 / 4.0);
        let t108 = t107 - 1.0;
        let t111 = t107 - 3.0 / 2.0 - 2.0 * t104 * t108;
        let t114 = t102 * t99 + 2.0 * t111 * t98;
        let t118 = piecewise3(t70, 1.0 / t73 / 36.0 - t77 / 960.0 + t80 / 26880.0 - t83 / 829440.0 + t86 / 28385280.0 - t89 / 1073479680.0 + t92 / 44590694400.0 - t95 / 2021444812800.0, 1.0 - 8.0 / 3.0 * t98 * t114);
        let t119 = rmath::pow(t59, 1.0 / 5.0);
        let t120 = 1.0 / t119;
        let t122 = 0.27 <= t69;
        let t123 = 0.27 < t69;
        let t124 = piecewise3(t123, t69, 0.27);
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
        let t160 = t132 / 3.3929038000650147e+37 - t137 / 3.511556992918352e+39 + 3.0 / 2240.0 / t126 - t141 / 11520.0 + 3.0 / 788480.0 * t143 - t146 / 7454720.0 + t148 / 247726080.0 - t150 / 9358540800.0 + t152 / 394474291200.0 - t155 / 18311911833600.0 + t158 / 927028425523200.0;
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
        let t193 = -t162 / 5.0785035485184e+16 + t165 / 2.991700272218112e+18 - t168 / 1.88514051721003e+20 + t171 / 1.2648942844388573e+22 - t174 / 9.002316741416457e+23 + t176 / 6.772652029299977e+25 - t179 / 5.36974553751641e+27 + t182 / 4.474731034888079e+29 - t185 / 3.909716563474291e+31 + t188 / 3.5738523369945735e+33 - t191 / 3.410951160703658e+35;
        let t195 = piecewise3(t123, 0.27, t69);
        let t196 = t195 * t195;
        let t198 = t196 * t196;
        let t199 = 64.0 * t198;
        let t200 = 20.0 * t196 - t199;
        let t203 = rmath::exp(-1.0 / t196 / 4.0);
        let t207 = 1.0 / t195;
        let t209 = rmath::erf(t207 / 2.0);
        let t212 = 10.0 * t195 * t209 * t99 + t200 * t203 - 36.0 * t196 + t199 - 3.0;
        let t216 = piecewise3(t122, t160 + t193, 24.0 * t196 * t212 + 1.0);
        let t217 = tau[ip] * t37;
        let t219 = 1.0 / t40 / rho[ip];
        let t225 = -0.14554132 * t217 * t219 + 0.043662396 * t46 * t33 + 0.04229627833333333 * t43;
        let t226 = t216 * t225;
        let t227 = t119 * t119;
        let t228 = 1.0 / t227;
        let t229 = t35 * t228;
        let t232 = 0.32 <= t69;
        let t233 = 0.32 < t69;
        let t234 = piecewise3(t233, t69, 0.32);
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
        let t290 = 3.0 / 7840.0 / t236 - t240 / 56448.0 + 5.0 / 8515584.0 * t243 - t246 / 61501440.0 + t249 / 2530344960.0 - t252 / 115811942400.0 + t255 / 5811921223680.0 - t258 / 316612955602944.0 + t261 / 1.85827061661696e+16 - t264 / 1.168055816159232e+18 + t267 / 7.824446865801216e+19 - t270 / 5.562511054710453e+21 + t273 / 4.181740504354862e+23 - t276 / 3.3139778504339334e+25 + t279 / 2.7608516801793436e+27 - t282 / 2.4119107039344544e+29 + t285 / 2.2046293272414373e+31 - t288 / 2.1042094544618633e+33;
        let t291 = piecewise3(t233, 0.32, t69);
        let t293 = t291 * t291;
        let t294 = t293 * t291;
        let t296 = t293 * t293;
        let t297 = t296 * t291;
        let t299 = t296 * t294;
        let t301 = t296 * t296;
        let t302 = t301 * t291;
        let t304 = -8.0 * t291 + 256.0 * t294 - 576.0 * t297 + 3840.0 * t299 - 122880.0 * t302;
        let t305 = 1.0 / t293;
        let t307 = rmath::exp(-t305 / 4.0);
        let t311 = t296 * t293;
        let t313 = -35.0 + 224.0 * t293 - 1440.0 * t296 + 5120.0 * t311;
        let t317 = -2.0 + 60.0 * t293;
        let t319 = 1.0 / t291;
        let t321 = rmath::erf(t319 / 2.0);
        let t324 = 2.0 * t317 * t321 * t99 + 24.0 * t294 * t313 + t304 * t307;
        let t328 = piecewise3(t232, t290, 1.0 + 8.0 / 7.0 * t291 * t324);
        let t329 = t328 * t30;
        let t330 = t329 * t34;
        let t332 = t38 * t42 * t228;
        let t335 = t118 * t120 + 35.0 / 81.0 * t226 * t229 + 0.026329605555555555 * t330 * t332;
        let t339 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t335);
        let tzk0 = 2.0 * t339;
        zk[ip] += tzk0;
    }
}
