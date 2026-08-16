//! GGA_X_LSPBE vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lspbe.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_lspbe_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_alpha: f64,
    param_kappa: f64,
    param_mu: f64,
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
        let t18 = piecewise5::<f64>(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3::<f64>(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3::<f64>(t19);
        let t25 = piecewise3::<f64>(t20, t22, t23 * t19);
        let t26 = pow_1_3::<f64>(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = param_mu * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3::<f64>(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t33 * sigma0;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3::<f64>(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t40 = t34 * t39;
        let t43 = param_kappa + t29 * t40 / 24.0;
        let t48 = param_kappa + 1.0;
        let t49 = param_alpha * t28;
        let t52 = f64::exp(-t49 * t40 / 24.0);
        let t55 = 1.0 + param_kappa * (1.0 - param_kappa / t43) - t48 * (1.0 - t52);
        let t59 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t55);
        let t60 = rho1 <= dens_threshold;
        let t61 = -t16;
        let t63 = piecewise5::<f64>(t14, t11, t10, t15, t61 * t7);
        let t64 = 1.0 + t63;
        let t65 = t64 <= zeta_threshold;
        let t66 = pow_1_3::<f64>(t64);
        let t68 = piecewise3::<f64>(t65, t22, t66 * t64);
        let t69 = t68 * t26;
        let t70 = t33 * sigma2;
        let t71 = rho1 * rho1;
        let t72 = pow_1_3::<f64>(rho1);
        let t73 = t72 * t72;
        let t75 = 1.0 / t73 / t71;
        let t76 = t70 * t75;
        let t79 = param_kappa + t29 * t76 / 24.0;
        let t86 = f64::exp(-t49 * t76 / 24.0);
        let t89 = 1.0 + param_kappa * (1.0 - param_kappa / t79) - t48 * (1.0 - t86);
        let t93 = piecewise3::<f64>(t60, 0.0, -3.0 / 8.0 * t5 * t69 * t89);
        let tzk0 = t59 + t93;
        zk[ip] += tzk0;
        let t94 = t6 * t6;
        let t95 = 1.0 / t94;
        let t96 = t16 * t95;
        let t98 = piecewise5::<f64>(t10, 0.0, t14, 0.0, t7 - t96);
        let t101 = piecewise3::<f64>(t20, 0.0, 4.0 / 3.0 * t23 * t98);
        let t102 = t101 * t26;
        let t106 = t26 * t26;
        let t107 = 1.0 / t106;
        let t108 = t25 * t107;
        let t111 = t5 * t108 * t55 / 8.0;
        let t112 = param_kappa * param_kappa;
        let t113 = t43 * t43;
        let t116 = t112 / t113 * param_mu;
        let t117 = t28 * t33;
        let t118 = t35 * rho0;
        let t120 = 1.0 / t37 / t118;
        let t125 = t48 * param_alpha * t28;
        let t130 = -t116 * t117 * sigma0 * t120 / 9.0 + t125 * t34 * t120 * t52 / 9.0;
        let t135 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t102 * t55 - t111 - 3.0 / 8.0 * t5 * t27 * t130);
        let t136 = t61 * t95;
        let t138 = piecewise5::<f64>(t14, 0.0, t10, 0.0, -t7 - t136);
        let t141 = piecewise3::<f64>(t65, 0.0, 4.0 / 3.0 * t66 * t138);
        let t142 = t141 * t26;
        let t146 = t68 * t107;
        let t149 = t5 * t146 * t89 / 8.0;
        let t151 = piecewise3::<f64>(t60, 0.0, -3.0 / 8.0 * t5 * t142 * t89 - t149);
        let tvrho0 = t59 + t93 + t6 * (t135 + t151);
        vrho[ip * 2] += tvrho0;
        let t155 = piecewise5::<f64>(t10, 0.0, t14, 0.0, -t7 - t96);
        let t158 = piecewise3::<f64>(t20, 0.0, 4.0 / 3.0 * t23 * t155);
        let t159 = t158 * t26;
        let t164 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t159 * t55 - t111);
        let t166 = piecewise5::<f64>(t14, 0.0, t10, 0.0, t7 - t136);
        let t169 = piecewise3::<f64>(t65, 0.0, 4.0 / 3.0 * t66 * t166);
        let t170 = t169 * t26;
        let t174 = t79 * t79;
        let t177 = t112 / t174 * param_mu;
        let t178 = t71 * rho1;
        let t180 = 1.0 / t73 / t178;
        let t188 = -t177 * t117 * sigma2 * t180 / 9.0 + t125 * t70 * t180 * t86 / 9.0;
        let t193 = piecewise3::<f64>(t60, 0.0, -3.0 / 8.0 * t5 * t170 * t89 - t149 - 3.0 / 8.0 * t5 * t69 * t188);
        let tvrho1 = t59 + t93 + t6 * (t164 + t193);
        vrho[ip * 2 + 1] += tvrho1;
        let t202 = -t125 * t33 * t39 * t52 / 24.0 + t116 * t117 * t39 / 24.0;
        let t206 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t202);
        let tvsigma0 = t6 * t206;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t213 = -t125 * t33 * t75 * t86 / 24.0 + t177 * t117 * t75 / 24.0;
        let t217 = piecewise3::<f64>(t60, 0.0, -3.0 / 8.0 * t5 * t69 * t213);
        let tvsigma2 = t6 * t217;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
