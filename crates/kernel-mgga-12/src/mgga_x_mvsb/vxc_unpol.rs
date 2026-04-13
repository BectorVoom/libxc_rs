//! MGGA_X_MVSB vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mvsb.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_mvsb_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_b: f64,
    param_c1: f64,
    param_e1: f64,
    param_k0: f64,
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
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = t7 * t18;
        let t20 = pow_1_3(rho[ip]);
        let t21 = M_CBRT2;
        let t22 = t21 * t21;
        let t23 = tau[ip] * t22;
        let t24 = t20 * t20;
        let t26 = 1.0 / t24 / rho[ip];
        let t27 = t23 * t26;
        let t28 = sigma[ip] * t22;
        let t29 = rho[ip] * rho[ip];
        let t31 = 1.0 / t24 / t29;
        let t34 = t27 - t28 * t31 / 8.0;
        let t35 = M_CBRT6;
        let t36 = t35 * t35;
        let t37 = M_PI * M_PI;
        let t38 = pow_1_3(t37);
        let t39 = t38 * t38;
        let t42 = t27 - 3.0 / 10.0 * t36 * t39;
        let t43 = 1.0 / t42;
        let t46 = param_k0 * (-t34 * t43 + 1.0);
        let t47 = t34 * t34;
        let t48 = param_e1 * t47;
        let t49 = t42 * t42;
        let t50 = 1.0 / t49;
        let t52 = t48 * t50 + 1.0;
        let t53 = t52 * t52;
        let t54 = t47 * t47;
        let t55 = param_c1 * t54;
        let t56 = t49 * t49;
        let t57 = 1.0 / t56;
        let t59 = t55 * t57 + t53;
        let t60 = pow_1_4(t59);
        let t61 = 1.0 / t60;
        let t63 = t46 * t61 + 1.0;
        let t67 = 1.0 / t38 / t37;
        let t69 = sigma[ip] * sigma[ip];
        let t71 = t29 * t29;
        let t72 = t71 * rho[ip];
        let t74 = 1.0 / t20 / t72;
        let t78 = 1.0 + param_b * t36 * t67 * t69 * t21 * t74 / 288.0;
        let t79 = f64::powf(t78, 1.0 / 8.0);
        let t80 = 1.0 / t79;
        let t84 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t63 * t80);
        let tzk0 = 2.0 * t84;
        zk[ip] += tzk0;
        let t85 = 1.0 / t24;
        let t90 = t23 * t31;
        let t92 = t29 * rho[ip];
        let t94 = 1.0 / t24 / t92;
        let t97 = -5.0 / 3.0 * t90 + t28 * t94 / 3.0;
        let t99 = t34 * t50;
        let t103 = param_k0 * (-t97 * t43 - 5.0 / 3.0 * t99 * t90);
        let t106 = 1.0 / t60 / t59;
        let t107 = param_e1 * t34;
        let t108 = t50 * t97;
        let t111 = t49 * t42;
        let t112 = 1.0 / t111;
        let t113 = t48 * t112;
        let t116 = 2.0 * t107 * t108 + 10.0 / 3.0 * t113 * t90;
        let t120 = param_c1 * t47 * t34;
        let t121 = t57 * t97;
        let t125 = 1.0 / t56 / t42;
        let t126 = t55 * t125;
        let t129 = 2.0 * t52 * t116 + 4.0 * t120 * t121 + 20.0 / 3.0 * t126 * t90;
        let t130 = t106 * t129;
        let t133 = t103 * t61 - t46 * t130 / 4.0;
        let t138 = t71 * t29;
        let t139 = 1.0 / t138;
        let t140 = t18 * t139;
        let t142 = t7 * t140 * t63;
        let t145 = 1.0 / t79 / t78 * param_b;
        let t146 = t145 * t36;
        let t149 = t146 * t67 * t69 * t21;
        let t153 = piecewise3(t3, 0.0, -t19 * t85 * t63 * t80 / 8.0 - 3.0 / 8.0 * t19 * t20 * t133 * t80 - t142 * t149 / 1152.0);
        let tvrho0 = 2.0 * rho[ip] * t153 + 2.0 * t84;
        vrho[ip] += tvrho0;
        let t156 = param_k0 * t22;
        let t157 = t31 * t43;
        let t161 = t52 * param_e1;
        let t162 = t161 * t34;
        let t163 = t50 * t22;
        let t164 = t163 * t31;
        let t166 = t57 * t22;
        let t167 = t166 * t31;
        let t168 = t120 * t167;
        let t170 = -t162 * t164 / 2.0 - t168 / 2.0;
        let t171 = t106 * t170;
        let t174 = t156 * t157 * t61 / 8.0 - t46 * t171 / 4.0;
        let t179 = 1.0 / t72;
        let t180 = t18 * t179;
        let t182 = t7 * t180 * t63;
        let t185 = t146 * t67 * sigma[ip] * t21;
        let t189 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t174 * t80 + t182 * t185 / 3072.0);
        let tvsigma0 = 2.0 * rho[ip] * t189;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t191 = t22 * t26;
        let t195 = param_k0 * (-t191 * t43 + t99 * t191);
        let t197 = t163 * t26;
        let t199 = t112 * t22;
        let t200 = t199 * t26;
        let t203 = 2.0 * t107 * t197 - 2.0 * t48 * t200;
        let t206 = t166 * t26;
        let t209 = t125 * t22;
        let t213 = -4.0 * t55 * t209 * t26 + 4.0 * t120 * t206 + 2.0 * t52 * t203;
        let t214 = t106 * t213;
        let t217 = t195 * t61 - t46 * t214 / 4.0;
        let t222 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t217 * t80);
        let tvtau0 = 2.0 * rho[ip] * t222;
        vtau[ip] += tvtau0;
    }
}
