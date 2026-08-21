//! GGA_C_OP_G96 vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_g96.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_op_g96_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 - rho1;
        let t2 = rho0 + rho1;
        let t3 = 1.0 / t2;
        let t4 = t1 * t3;
        let t5 = rmath::abs(t4);
        let t11 = 1.0 - t5 <= zeta_threshold || rho0 <= dens_threshold && rho1 <= dens_threshold;
        let t13 = 1.0 + t4 <= zeta_threshold;
        let t14 = zeta_threshold - 1.0;
        let t16 = 1.0 - t4 <= zeta_threshold;
        let t17 = -t14;
        let t18 = piecewise5(t13, t14, t16, t17, t4);
        let t19 = t18 * t18;
        let t20 = 1.0 - t19;
        let t21 = t20 * t2;
        let t24 = 2.0 * rho0 * t3 <= zeta_threshold;
        let t27 = 2.0 * rho1 * t3 <= zeta_threshold;
        let t28 = piecewise5(t24, t14, t27, t17, t4);
        let t29 = 1.0 + t28;
        let t32 = t29 * t2 / 2.0 <= dens_threshold;
        let t33 = M_CBRT3;
        let t34 = t33 * t33;
        let t36 = pow_1_3(1.0 / M_PI);
        let t37 = 1.0 / t36;
        let t38 = t34 * t37;
        let t39 = M_CBRT4;
        let t40 = t38 * t39;
        let t41 = M_CBRT2;
        let t42 = t29 <= zeta_threshold;
        let t43 = 1.0 - t28;
        let t44 = t43 <= zeta_threshold;
        let t45 = piecewise5(t42, t14, t44, t17, t28);
        let t46 = 1.0 + t45;
        let t47 = t46 * t2;
        let t48 = pow_1_3(t47);
        let t49 = 1.0 / t48;
        let t51 = rmath::sqrt(sigma0);
        let t52 = pow_1_3(rho0);
        let t54 = 1.0 / t52 / rho0;
        let t55 = t51 * t54;
        let t56 = rmath::sqrt(t55);
        let t57 = t56 * t55;
        let t61 = 1.0 + 2.0 / 1233.0 * t38 * t39 * t57;
        let t62 = 1.0 / t61;
        let t66 = piecewise3(t32, 0.0, t40 * t41 * t49 * t62 / 9.0);
        let t70 = t43 * t2 / 2.0 <= dens_threshold;
        let t71 = piecewise5(t44, t14, t42, t17, -t28);
        let t72 = 1.0 + t71;
        let t73 = t72 * t2;
        let t74 = pow_1_3(t73);
        let t75 = 1.0 / t74;
        let t77 = rmath::sqrt(sigma2);
        let t78 = pow_1_3(rho1);
        let t80 = 1.0 / t78 / rho1;
        let t81 = t77 * t80;
        let t82 = rmath::sqrt(t81);
        let t83 = t82 * t81;
        let t87 = 1.0 + 2.0 / 1233.0 * t38 * t39 * t83;
        let t88 = 1.0 / t87;
        let t92 = piecewise3(t70, 0.0, t40 * t41 * t75 * t88 / 9.0);
        let t93 = t66 + t92;
        let t94 = t93 == 0.0;
        let t95 = piecewise3(t94, f64::EPSILON, t93);
        let t98 = 3.59628532 / t95 + 0.5764;
        let t99 = t95 * t95;
        let t100 = t99 * t99;
        let t101 = 1.0 / t100;
        let t103 = t99 * t95;
        let t104 = 1.0 / t103;
        let t106 = 1.0 / t99;
        let t108 = 31.220719919544194 * t101 + 14.903739892213245 * t104 + 1.778517305052 * t106;
        let t109 = 1.0 / t108;
        let t110 = t98 * t109;
        let tzk0 = piecewise3(t11, 0.0, -0.25 * t21 * t110);
        zk[ip] += tzk0;
        let t113 = t2 * t2;
        let t114 = 1.0 / t113;
        let t115 = t1 * t114;
        let t116 = t3 - t115;
        let t117 = piecewise5(t13, 0.0, t16, 0.0, t116);
        let t118 = t18 * t117;
        let t119 = t2 * t98;
        let t120 = t119 * t109;
        let t123 = t20 * t98;
        let t125 = 0.25 * t123 * t109;
        let t127 = 1.0 / t48 / t47;
        let t128 = t41 * t127;
        let t129 = piecewise5(t24, 0.0, t27, 0.0, t116);
        let t130 = piecewise5(t42, 0.0, t44, 0.0, t129);
        let t132 = t130 * t2 + t45 + 1.0;
        let t137 = t36 * t36;
        let t138 = 1.0 / t137;
        let t139 = t33 * t138;
        let t140 = t39 * t39;
        let t141 = t140 * t41;
        let t142 = t139 * t141;
        let t143 = t61 * t61;
        let t144 = 1.0 / t143;
        let t145 = t49 * t144;
        let t146 = t56 * t51;
        let t147 = rho0 * rho0;
        let t149 = 1.0 / t52 / t147;
        let t150 = t146 * t149;
        let t155 = piecewise3(t32, 0.0, -t40 * t128 * t62 * t132 / 27.0 + 4.0 / 3699.0 * t142 * t145 * t150);
        let t157 = 1.0 / t74 / t73;
        let t158 = t41 * t157;
        let t159 = piecewise5(t44, 0.0, t42, 0.0, -t129);
        let t161 = t159 * t2 + t71 + 1.0;
        let t166 = piecewise3(t70, 0.0, -t40 * t158 * t88 * t161 / 27.0);
        let t168 = piecewise3(t94, 0.0, t155 + t166);
        let t169 = t106 * t168;
        let t170 = t169 * t109;
        let t173 = t108 * t108;
        let t174 = 1.0 / t173;
        let t175 = t98 * t174;
        let t177 = 1.0 / t100 / t95;
        let t178 = t177 * t168;
        let t180 = t101 * t168;
        let t184 = -124.88287967817678 * t178 - 44.711219676639736 * t180 - 3.557034610104 * t104 * t168;
        let t185 = t175 * t184;
        let t189 = piecewise3(t11, 0.0, 0.5 * t118 * t120 - t125 + 0.89907133 * t21 * t170 + 0.25 * t21 * t185);
        let tvrho0 = t2 * t189 + tzk0;
        vrho[ip * 2] += tvrho0;
        let t191 = -t3 - t115;
        let t192 = piecewise5(t13, 0.0, t16, 0.0, t191);
        let t193 = t18 * t192;
        let t196 = piecewise5(t24, 0.0, t27, 0.0, t191);
        let t197 = piecewise5(t42, 0.0, t44, 0.0, t196);
        let t199 = t197 * t2 + t45 + 1.0;
        let t204 = piecewise3(t32, 0.0, -t40 * t128 * t62 * t199 / 27.0);
        let t205 = piecewise5(t44, 0.0, t42, 0.0, -t196);
        let t207 = t205 * t2 + t71 + 1.0;
        let t212 = t87 * t87;
        let t213 = 1.0 / t212;
        let t214 = t75 * t213;
        let t215 = t82 * t77;
        let t216 = rho1 * rho1;
        let t218 = 1.0 / t78 / t216;
        let t219 = t215 * t218;
        let t224 = piecewise3(t70, 0.0, -t40 * t158 * t88 * t207 / 27.0 + 4.0 / 3699.0 * t142 * t214 * t219);
        let t226 = piecewise3(t94, 0.0, t204 + t224);
        let t227 = t106 * t226;
        let t228 = t227 * t109;
        let t231 = t177 * t226;
        let t233 = t101 * t226;
        let t235 = t104 * t226;
        let t237 = -124.88287967817678 * t231 - 44.711219676639736 * t233 - 3.557034610104 * t235;
        let t238 = t175 * t237;
        let t242 = piecewise3(t11, 0.0, 0.5 * t193 * t120 - t125 + 0.89907133 * t21 * t228 + 0.25 * t21 * t238);
        let tvrho1 = t2 * t242 + tzk0;
        vrho[ip * 2 + 1] += tvrho1;
        let t244 = 1.0 / t51;
        let t245 = t56 * t244;
        let t250 = piecewise3(t32, 0.0, -t142 * t145 * t245 * t54 / 2466.0);
        let t251 = piecewise3(t94, 0.0, t250);
        let t252 = t106 * t251;
        let t253 = t252 * t109;
        let t256 = t177 * t251;
        let t258 = t101 * t251;
        let t260 = t104 * t251;
        let t262 = -124.88287967817678 * t256 - 44.711219676639736 * t258 - 3.557034610104 * t260;
        let t263 = t175 * t262;
        let t267 = piecewise3(t11, 0.0, 0.89907133 * t21 * t253 + 0.25 * t21 * t263);
        let tvsigma0 = t2 * t267;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t268 = 1.0 / t77;
        let t269 = t82 * t268;
        let t274 = piecewise3(t70, 0.0, -t142 * t214 * t269 * t80 / 2466.0);
        let t275 = piecewise3(t94, 0.0, t274);
        let t276 = t106 * t275;
        let t277 = t276 * t109;
        let t280 = t177 * t275;
        let t282 = t101 * t275;
        let t284 = t104 * t275;
        let t286 = -124.88287967817678 * t280 - 44.711219676639736 * t282 - 3.557034610104 * t284;
        let t287 = t175 * t286;
        let t291 = piecewise3(t11, 0.0, 0.89907133 * t21 * t277 + 0.25 * t21 * t287);
        let tvsigma2 = t2 * t291;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
