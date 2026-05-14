//! GGA_X_HERMAN vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_herman.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_herman_vxc_pol(
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
        let t28 = t2 * t2;
        let t30 = pow_1_3(1.0 / M_PI);
        let t31 = 1.0 / t30;
        let t32 = t28 * t31;
        let t33 = M_CBRT4;
        let t34 = t33 * sigma0;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t43 = 1.0 + 0.66666666666666666668e-3 * t32 * t34 * t39;
        let t47 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t43);
        let t48 = rho1 <= dens_threshold;
        let t49 = -t16;
        let t51 = piecewise5(t14, t11, t10, t15, t49 * t7);
        let t52 = 1.0 + t51;
        let t53 = t52 <= zeta_threshold;
        let t54 = pow_1_3(t52);
        let t56 = piecewise3(t53, t22, t54 * t52);
        let t57 = t56 * t26;
        let t58 = t33 * sigma2;
        let t59 = rho1 * rho1;
        let t60 = pow_1_3(rho1);
        let t61 = t60 * t60;
        let t63 = 1.0 / t61 / t59;
        let t67 = 1.0 + 0.66666666666666666668e-3 * t32 * t58 * t63;
        let t71 = piecewise3(t48, 0.0, -3.0 / 8.0 * t5 * t57 * t67);
        let tzk0 = t47 + t71;
        zk[ip] += tzk0;
        let t72 = t6 * t6;
        let t73 = 1.0 / t72;
        let t74 = t16 * t73;
        let t76 = piecewise5(t10, 0.0, t14, 0.0, t7 - t74);
        let t79 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t76);
        let t80 = t79 * t26;
        let t84 = t26 * t26;
        let t85 = 1.0 / t84;
        let t86 = t25 * t85;
        let t89 = t5 * t86 * t43 / 8.0;
        let t90 = t27 * t31;
        let t93 = 1.0 / t37 / t35 / rho0;
        let t94 = t34 * t93;
        let t98 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t80 * t43 - t89 + 0.13655681265105913629e-2 * t90 * t94);
        let t99 = t49 * t73;
        let t101 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t99);
        let t104 = piecewise3(t53, 0.0, 4.0 / 3.0 * t54 * t101);
        let t105 = t104 * t26;
        let t109 = t56 * t85;
        let t112 = t5 * t109 * t67 / 8.0;
        let t114 = piecewise3(t48, 0.0, -3.0 / 8.0 * t5 * t105 * t67 - t112);
        let tvrho0 = t47 + t71 + t6 * (t98 + t114);
        vrho[ip * 2] += tvrho0;
        let t118 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t74);
        let t121 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t118);
        let t122 = t121 * t26;
        let t127 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t122 * t43 - t89);
        let t129 = piecewise5(t14, 0.0, t10, 0.0, t7 - t99);
        let t132 = piecewise3(t53, 0.0, 4.0 / 3.0 * t54 * t129);
        let t133 = t132 * t26;
        let t137 = t57 * t31;
        let t140 = 1.0 / t61 / t59 / rho1;
        let t141 = t58 * t140;
        let t145 = piecewise3(t48, 0.0, -3.0 / 8.0 * t5 * t133 * t67 - t112 + 0.13655681265105913629e-2 * t137 * t141);
        let tvrho1 = t47 + t71 + t6 * (t127 + t145);
        vrho[ip * 2 + 1] += tvrho1;
        let t148 = t31 * t33;
        let t149 = t148 * t39;
        let t152 = piecewise3(t1, 0.0, -0.51208804744147176112e-3 * t27 * t149);
        let tvsigma0 = t6 * t152;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t153 = t148 * t63;
        let t156 = piecewise3(t48, 0.0, -0.51208804744147176112e-3 * t57 * t153);
        let tvsigma2 = t6 * t156;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
