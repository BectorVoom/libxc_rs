//! GGA_X_LAG vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lag.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_lag_vxc_pol(
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
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t7 = 2.0 * rho0 * t4 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t11 = 2.0 * rho1 * t4 <= zeta_threshold;
        let t12 = -t8;
        let t13 = rho0 - rho1;
        let t15 = piecewise5(t7, t8, t11, t12, t13 * t4);
        let t16 = 1.0 + t15;
        let t17 = t16 <= zeta_threshold;
        let t18 = pow_1_3(zeta_threshold);
        let t19 = t18 * zeta_threshold;
        let t20 = pow_1_3(t16);
        let t22 = piecewise3(t17, t19, t20 * t16);
        let t23 = t2 * t22;
        let t24 = pow_1_3(t3);
        let t25 = M_CBRT6;
        let t26 = t25 * t25;
        let t27 = M_PI * M_PI;
        let t28 = pow_1_3(t27);
        let t29 = 1.0 / t28;
        let t30 = t26 * t29;
        let t31 = f64::sqrt(sigma0);
        let t32 = pow_1_3(rho0);
        let t34 = 1.0 / t32 / rho0;
        let t36 = t30 * t31 * t34;
        let t37 = f64::powf(t36, 0.2626712e1);
        let t40 = 1.0 + 0.13471619689594796103e-3 * t37;
        let t41 = f64::powf(t40, -0.657946e0);
        let t42 = t24 * t37 * t41;
        let t45 = piecewise3(t1, 0.0, -0.15400028771927569605e-4 * t23 * t42);
        let t46 = rho1 <= dens_threshold;
        let t47 = -t13;
        let t49 = piecewise5(t11, t8, t7, t12, t47 * t4);
        let t50 = 1.0 + t49;
        let t51 = t50 <= zeta_threshold;
        let t52 = pow_1_3(t50);
        let t54 = piecewise3(t51, t19, t52 * t50);
        let t55 = t2 * t54;
        let t56 = f64::sqrt(sigma2);
        let t57 = pow_1_3(rho1);
        let t59 = 1.0 / t57 / rho1;
        let t61 = t30 * t56 * t59;
        let t62 = f64::powf(t61, 0.2626712e1);
        let t65 = 1.0 + 0.13471619689594796103e-3 * t62;
        let t66 = f64::powf(t65, -0.657946e0);
        let t67 = t24 * t62 * t66;
        let t70 = piecewise3(t46, 0.0, -0.15400028771927569605e-4 * t55 * t67);
        let tzk0 = t45 + t70;
        zk[ip] += tzk0;
        let t71 = t3 * t3;
        let t72 = 1.0 / t71;
        let t73 = t13 * t72;
        let t75 = piecewise5(t7, 0.0, t11, 0.0, t4 - t73);
        let t78 = piecewise3(t17, 0.0, 4.0 / 3.0 * t20 * t75);
        let t79 = t2 * t78;
        let t82 = t24 * t24;
        let t83 = 1.0 / t82;
        let t85 = t83 * t37 * t41;
        let t87 = 0.5133342923975856535e-5 * t23 * t85;
        let t88 = f64::powf(t36, 0.1626712e1);
        let t89 = t24 * t88;
        let t90 = t23 * t89;
        let t91 = t41 * t26;
        let t92 = t29 * t31;
        let t93 = rho0 * rho0;
        let t95 = 1.0 / t32 / t93;
        let t96 = t92 * t95;
        let t97 = t91 * t96;
        let t100 = f64::powf(t36, 0.4253424e1);
        let t101 = t24 * t100;
        let t102 = t23 * t101;
        let t103 = f64::powf(t40, -0.1657946e1);
        let t104 = t103 * t26;
        let t105 = t104 * t96;
        let t109 = piecewise3(t1, 0.0, -0.15400028771927569605e-4 * t79 * t42 - t87 + 0.53935253834089880284e-4 * t90 * t97 - 0.47806042356233315032e-8 * t102 * t105);
        let t110 = t47 * t72;
        let t112 = piecewise5(t11, 0.0, t7, 0.0, -t4 - t110);
        let t115 = piecewise3(t51, 0.0, 4.0 / 3.0 * t52 * t112);
        let t116 = t2 * t115;
        let t120 = t83 * t62 * t66;
        let t122 = 0.5133342923975856535e-5 * t55 * t120;
        let t124 = piecewise3(t46, 0.0, -0.15400028771927569605e-4 * t116 * t67 - t122);
        let tvrho0 = t45 + t70 + t3 * (t109 + t124);
        vrho[ip * 2] += tvrho0;
        let t128 = piecewise5(t7, 0.0, t11, 0.0, -t4 - t73);
        let t131 = piecewise3(t17, 0.0, 4.0 / 3.0 * t20 * t128);
        let t132 = t2 * t131;
        let t136 = piecewise3(t1, 0.0, -0.15400028771927569605e-4 * t132 * t42 - t87);
        let t138 = piecewise5(t11, 0.0, t7, 0.0, t4 - t110);
        let t141 = piecewise3(t51, 0.0, 4.0 / 3.0 * t52 * t138);
        let t142 = t2 * t141;
        let t145 = f64::powf(t61, 0.1626712e1);
        let t146 = t24 * t145;
        let t147 = t55 * t146;
        let t148 = t66 * t26;
        let t149 = t29 * t56;
        let t150 = rho1 * rho1;
        let t152 = 1.0 / t57 / t150;
        let t153 = t149 * t152;
        let t154 = t148 * t153;
        let t157 = f64::powf(t61, 0.4253424e1);
        let t158 = t24 * t157;
        let t159 = t55 * t158;
        let t160 = f64::powf(t65, -0.1657946e1);
        let t161 = t160 * t26;
        let t162 = t161 * t153;
        let t166 = piecewise3(t46, 0.0, -0.15400028771927569605e-4 * t142 * t67 - t122 + 0.53935253834089880284e-4 * t147 * t154 - 0.47806042356233315032e-8 * t159 * t162);
        let tvrho1 = t45 + t70 + t3 * (t136 + t166);
        vrho[ip * 2 + 1] += tvrho1;
        let t169 = 1.0 / t31;
        let t170 = t29 * t169;
        let t171 = t170 * t34;
        let t172 = t91 * t171;
        let t175 = t104 * t171;
        let t179 = piecewise3(t1, 0.0, -0.20225720187783705106e-4 * t90 * t172 + 0.17927265883587493137e-8 * t102 * t175);
        let tvsigma0 = t3 * t179;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t180 = 1.0 / t56;
        let t181 = t29 * t180;
        let t182 = t181 * t59;
        let t183 = t148 * t182;
        let t186 = t161 * t182;
        let t190 = piecewise3(t46, 0.0, -0.20225720187783705106e-4 * t147 * t183 + 0.17927265883587493137e-8 * t159 * t186);
        let tvsigma2 = t3 * t190;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
