//! GGA_X_RPBE vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_rpbe.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_rpbe_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_rpbe_kappa: f64,
    param_rpbe_mu: f64,
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
        let t29 = param_rpbe_mu * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t41 = 1.0 / param_rpbe_kappa;
        let t45 = f64::exp(-t34 * sigma0 * t39 * t41 / 24.0);
        let t48 = 1.0 + param_rpbe_kappa * (1.0 - t45);
        let t52 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t48);
        let t53 = rho1 <= dens_threshold;
        let t54 = -t16;
        let t56 = piecewise5(t14, t11, t10, t15, t54 * t7);
        let t57 = 1.0 + t56;
        let t58 = t57 <= zeta_threshold;
        let t59 = pow_1_3(t57);
        let t61 = piecewise3(t58, t22, t59 * t57);
        let t62 = t61 * t26;
        let t63 = rho1 * rho1;
        let t64 = pow_1_3(rho1);
        let t65 = t64 * t64;
        let t67 = 1.0 / t65 / t63;
        let t72 = f64::exp(-t34 * sigma2 * t67 * t41 / 24.0);
        let t75 = 1.0 + param_rpbe_kappa * (1.0 - t72);
        let t79 = piecewise3(t53, 0.0, -3.0 / 8.0 * t5 * t62 * t75);
        let tzk0 = t52 + t79;
        zk[ip] += tzk0;
        let t80 = t6 * t6;
        let t81 = 1.0 / t80;
        let t82 = t16 * t81;
        let t84 = piecewise5(t10, 0.0, t14, 0.0, t7 - t82);
        let t87 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t84);
        let t88 = t87 * t26;
        let t92 = t26 * t26;
        let t93 = 1.0 / t92;
        let t94 = t25 * t93;
        let t97 = t5 * t94 * t48 / 8.0;
        let t99 = t5 * t27 * param_rpbe_mu;
        let t100 = t28 * t33;
        let t101 = t35 * rho0;
        let t103 = 1.0 / t37 / t101;
        let t106 = t100 * sigma0 * t103 * t45;
        let t110 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t88 * t48 - t97 + t99 * t106 / 24.0);
        let t111 = t54 * t81;
        let t113 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t111);
        let t116 = piecewise3(t58, 0.0, 4.0 / 3.0 * t59 * t113);
        let t117 = t116 * t26;
        let t121 = t61 * t93;
        let t124 = t5 * t121 * t75 / 8.0;
        let t126 = piecewise3(t53, 0.0, -3.0 / 8.0 * t5 * t117 * t75 - t124);
        let tvrho0 = t52 + t79 + t6 * (t110 + t126);
        vrho[ip * 2] += tvrho0;
        let t130 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t82);
        let t133 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t130);
        let t134 = t133 * t26;
        let t139 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t134 * t48 - t97);
        let t141 = piecewise5(t14, 0.0, t10, 0.0, t7 - t111);
        let t144 = piecewise3(t58, 0.0, 4.0 / 3.0 * t59 * t141);
        let t145 = t144 * t26;
        let t150 = t5 * t62 * param_rpbe_mu;
        let t151 = t63 * rho1;
        let t153 = 1.0 / t65 / t151;
        let t156 = t100 * sigma2 * t153 * t72;
        let t160 = piecewise3(t53, 0.0, -3.0 / 8.0 * t5 * t145 * t75 - t124 + t150 * t156 / 24.0);
        let tvrho1 = t52 + t79 + t6 * (t139 + t160);
        vrho[ip * 2 + 1] += tvrho1;
        let t163 = t5 * t27;
        let t166 = t29 * t33 * t39 * t45;
        let t169 = piecewise3(t1, 0.0, -t163 * t166 / 64.0);
        let tvsigma0 = t6 * t169;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t170 = t5 * t62;
        let t173 = t29 * t33 * t67 * t72;
        let t176 = piecewise3(t53, 0.0, -t170 * t173 / 64.0);
        let tvsigma2 = t6 * t176;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
