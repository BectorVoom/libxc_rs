//! GGA_X_C09X vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_c09x.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_c09x_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = M_PI * M_PI;
        let t30 = pow_1_3(t29);
        let t31 = t30 * t30;
        let t32 = 1.0 / t31;
        let t33 = t28 * t32;
        let t34 = rho0 * rho0;
        let t35 = pow_1_3(rho0);
        let t36 = t35 * t35;
        let t38 = 1.0 / t36 / t34;
        let t39 = sigma0 * t38;
        let t40 = t33 * t39;
        let t42 = f64::exp(-0.20125e-2 * t40);
        let t47 = f64::exp(-0.100625e-2 * t40);
        let t49 = 0.2245e1 + 0.25708333333333333333e-2 * t33 * t39 * t42 - 0.1245e1 * t47;
        let t53 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t49);
        let t54 = rho1 <= dens_threshold;
        let t55 = -t16;
        let t57 = piecewise5(t14, t11, t10, t15, t55 * t7);
        let t58 = 1.0 + t57;
        let t59 = t58 <= zeta_threshold;
        let t60 = pow_1_3(t58);
        let t62 = piecewise3(t59, t22, t60 * t58);
        let t63 = t62 * t26;
        let t64 = rho1 * rho1;
        let t65 = pow_1_3(rho1);
        let t66 = t65 * t65;
        let t68 = 1.0 / t66 / t64;
        let t69 = sigma2 * t68;
        let t70 = t33 * t69;
        let t72 = f64::exp(-0.20125e-2 * t70);
        let t77 = f64::exp(-0.100625e-2 * t70);
        let t79 = 0.2245e1 + 0.25708333333333333333e-2 * t33 * t69 * t72 - 0.1245e1 * t77;
        let t83 = piecewise3(t54, 0.0, -3.0 / 8.0 * t5 * t63 * t79);
        let tzk0 = t53 + t83;
        zk[ip] += tzk0;
        let t84 = t6 * t6;
        let t85 = 1.0 / t84;
        let t86 = t16 * t85;
        let t88 = piecewise5(t10, 0.0, t14, 0.0, t7 - t86);
        let t91 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t88);
        let t92 = t91 * t26;
        let t96 = t26 * t26;
        let t97 = 1.0 / t96;
        let t98 = t25 * t97;
        let t101 = t5 * t98 * t49 / 8.0;
        let t102 = t34 * rho0;
        let t104 = 1.0 / t36 / t102;
        let t105 = sigma0 * t104;
        let t109 = t28 * t28;
        let t112 = t109 / t30 / t29;
        let t113 = sigma0 * sigma0;
        let t114 = t34 * t34;
        let t115 = t114 * t34;
        let t117 = 1.0 / t35 / t115;
        let t125 = -0.68555555555555555555e-2 * t33 * t105 * t42 + 0.13796805555555555555e-4 * t112 * t113 * t117 * t42 - 0.334075e-2 * t33 * t105 * t47;
        let t130 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t92 * t49 - t101 - 3.0 / 8.0 * t5 * t27 * t125);
        let t131 = t55 * t85;
        let t133 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t131);
        let t136 = piecewise3(t59, 0.0, 4.0 / 3.0 * t60 * t133);
        let t137 = t136 * t26;
        let t141 = t62 * t97;
        let t144 = t5 * t141 * t79 / 8.0;
        let t146 = piecewise3(t54, 0.0, -3.0 / 8.0 * t5 * t137 * t79 - t144);
        let tvrho0 = t53 + t83 + t6 * (t130 + t146);
        vrho[ip * 2] += tvrho0;
        let t150 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t86);
        let t153 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t150);
        let t154 = t153 * t26;
        let t159 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t154 * t49 - t101);
        let t161 = piecewise5(t14, 0.0, t10, 0.0, t7 - t131);
        let t164 = piecewise3(t59, 0.0, 4.0 / 3.0 * t60 * t161);
        let t165 = t164 * t26;
        let t169 = t64 * rho1;
        let t171 = 1.0 / t66 / t169;
        let t172 = sigma2 * t171;
        let t176 = sigma2 * sigma2;
        let t177 = t64 * t64;
        let t178 = t177 * t64;
        let t180 = 1.0 / t65 / t178;
        let t188 = -0.68555555555555555555e-2 * t33 * t172 * t72 + 0.13796805555555555555e-4 * t112 * t176 * t180 * t72 - 0.334075e-2 * t33 * t172 * t77;
        let t193 = piecewise3(t54, 0.0, -3.0 / 8.0 * t5 * t165 * t79 - t144 - 3.0 / 8.0 * t5 * t63 * t188);
        let tvrho1 = t53 + t83 + t6 * (t159 + t193);
        vrho[ip * 2 + 1] += tvrho1;
        let t199 = t114 * rho0;
        let t201 = 1.0 / t35 / t199;
        let t209 = 0.25708333333333333333e-2 * t33 * t38 * t42 - 0.51738020833333333333e-5 * t112 * sigma0 * t201 * t42 + 0.125278125e-2 * t33 * t38 * t47;
        let t213 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t209);
        let tvsigma0 = t6 * t213;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t217 = t177 * rho1;
        let t219 = 1.0 / t65 / t217;
        let t227 = 0.25708333333333333333e-2 * t33 * t68 * t72 - 0.51738020833333333333e-5 * t112 * sigma2 * t219 * t72 + 0.125278125e-2 * t33 * t68 * t77;
        let t231 = piecewise3(t54, 0.0, -3.0 / 8.0 * t5 * t63 * t227);
        let tvsigma2 = t6 * t231;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
