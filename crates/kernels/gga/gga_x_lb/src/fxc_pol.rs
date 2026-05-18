//! GGA_X_LB fxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_vxc/gga_x_lb.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_lb_fxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_alpha: f64,
    param_beta: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < vrho.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = M_CBRT3;
        let t4 = pow_1_3::<f64>(1.0 / M_PI);
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t9 = param_alpha * t1 * t4 * t6 / 2.0;
        let t10 = f64::sqrt(sigma0);
        let t11 = pow_1_3::<f64>(rho0);
        let t13 = 1.0 / t11 / rho0;
        let t14 = t10 * t13;
        let t15 = t14 < 300.0;
        let t16 = param_beta * sigma0;
        let t17 = rho0 * rho0;
        let t18 = t11 * t11;
        let t20 = 1.0 / t18 / t17;
        let t21 = param_beta * t10;
        let t23 = param_gamma * t10 * t13;
        let t24 = f64::ln(t23 + f64::sqrt(t23 * t23 + 1.0));
        let t25 = t13 * t24;
        let t28 = 3.0 * t21 * t25 + 1.0;
        let t29 = 1.0 / t28;
        let t33 = f64::ln(2.0 * t23);
        let t34 = 1.0 / t33;
        let t37 = piecewise3::<f64>(t15, t16 * t20 * t29, t14 * t34 / 3.0);
        let t38 = -t9 - t37;
        let tvrho0 = t38 * t11;
        vrho[ip * 2] += tvrho0;
        let t39 = f64::sqrt(sigma2);
        let t40 = pow_1_3::<f64>(rho1);
        let t42 = 1.0 / t40 / rho1;
        let t43 = t39 * t42;
        let t44 = t43 < 300.0;
        let t45 = param_beta * sigma2;
        let t46 = rho1 * rho1;
        let t47 = t40 * t40;
        let t49 = 1.0 / t47 / t46;
        let t50 = param_beta * t39;
        let t52 = param_gamma * t39 * t42;
        let t53 = f64::ln(t52 + f64::sqrt(t52 * t52 + 1.0));
        let t54 = t42 * t53;
        let t57 = 3.0 * t50 * t54 + 1.0;
        let t58 = 1.0 / t57;
        let t62 = f64::ln(2.0 * t52);
        let t63 = 1.0 / t62;
        let t66 = piecewise3::<f64>(t44, t45 * t49 * t58, t43 * t63 / 3.0);
        let t67 = -t9 - t66;
        let tvrho1 = t67 * t40;
        vrho[ip * 2 + 1] += tvrho1;
        let t68 = t17 * rho0;
        let t70 = 1.0 / t18 / t68;
        let t74 = t28 * t28;
        let t75 = 1.0 / t74;
        let t76 = t20 * t75;
        let t78 = 1.0 / t11 / t17;
        let t79 = t78 * t24;
        let t82 = param_gamma * param_gamma;
        let t85 = t82 * sigma0 * t20 + 1.0;
        let t86 = f64::sqrt(t85);
        let t87 = 1.0 / t86;
        let t88 = t70 * param_gamma * t87;
        let t91 = -4.0 * t16 * t88 - 4.0 * t21 * t79;
        let t95 = t10 * t78;
        let t97 = t33 * t33;
        let t98 = 1.0 / t97;
        let t102 = piecewise3::<f64>(t15, -8.0 / 3.0 * t16 * t70 * t29 - t16 * t76 * t91, -4.0 / 9.0 * t95 * t34 + 4.0 / 9.0 * t95 * t98);
        let t104 = 1.0 / t18;
        let tv2rho20 = -t102 * t11 + t38 * t104 / 3.0;
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = 0.0;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t107 = t46 * rho1;
        let t109 = 1.0 / t47 / t107;
        let t113 = t57 * t57;
        let t114 = 1.0 / t113;
        let t115 = t49 * t114;
        let t117 = 1.0 / t40 / t46;
        let t118 = t117 * t53;
        let t123 = t82 * sigma2 * t49 + 1.0;
        let t124 = f64::sqrt(t123);
        let t125 = 1.0 / t124;
        let t126 = t109 * param_gamma * t125;
        let t129 = -4.0 * t50 * t118 - 4.0 * t45 * t126;
        let t133 = t39 * t117;
        let t135 = t62 * t62;
        let t136 = 1.0 / t135;
        let t140 = piecewise3::<f64>(t44, -8.0 / 3.0 * t45 * t109 * t58 - t45 * t115 * t129, 4.0 / 9.0 * t133 * t136 - 4.0 / 9.0 * t133 * t63);
        let t142 = 1.0 / t47;
        let tv2rho22 = -t140 * t40 + t67 * t142 / 3.0;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t145 = param_beta * t20;
        let t147 = 1.0 / t10;
        let t148 = param_beta * t147;
        let t150 = param_gamma * t87;
        let t153 = 3.0 / 2.0 * t145 * t150 + 3.0 / 2.0 * t148 * t25;
        let t157 = t147 * t13;
        let t162 = piecewise3::<f64>(t15, -t16 * t76 * t153 + t145 * t29, t157 * t34 / 6.0 - t157 * t98 / 6.0);
        let tv2rhosigma0 = -t162 * t11;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let tv2rhosigma1 = 0.0;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let tv2rhosigma2 = 0.0;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let tv2rhosigma3 = 0.0;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = 0.0;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let t164 = param_beta * t49;
        let t166 = 1.0 / t39;
        let t167 = param_beta * t166;
        let t169 = param_gamma * t125;
        let t172 = 3.0 / 2.0 * t164 * t169 + 3.0 / 2.0 * t167 * t54;
        let t176 = t166 * t42;
        let t181 = piecewise3::<f64>(t44, -t45 * t115 * t172 + t164 * t58, -t176 * t136 / 6.0 + t176 * t63 / 6.0);
        let tv2rhosigma5 = -t181 * t40;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
    }
}
