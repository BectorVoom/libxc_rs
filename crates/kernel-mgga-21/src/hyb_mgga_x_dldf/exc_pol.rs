//! HYB_MGGA_X_DLDF exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_x_dldf.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn hyb_mgga_x_dldf_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
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
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = rho0 + rho1;
        let t5 = 1.0 / t4;
        let t8 = 2.0 * rho0 * t5 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t12 = 2.0 * rho1 * t5 <= zeta_threshold;
        let t13 = -t9;
        let t14 = rho0 - rho1;
        let t16 = piecewise5(t8, t9, t12, t13, t14 * t5);
        let t17 = 1.0 + t16;
        let t18 = t17 <= zeta_threshold;
        let t19 = pow_1_3(zeta_threshold);
        let t20 = t19 * zeta_threshold;
        let t21 = pow_1_3(t17);
        let t23 = piecewise3(t18, t20, t21 * t17);
        let t24 = t3 * t23;
        let t25 = pow_1_3(t4);
        let t26 = M_CBRT6;
        let t27 = M_PI * M_PI;
        let t28 = pow_1_3(t27);
        let t29 = t28 * t28;
        let t30 = 1.0 / t29;
        let t31 = t26 * t30;
        let t32 = rho0 * rho0;
        let t33 = pow_1_3(rho0);
        let t34 = t33 * t33;
        let t36 = 1.0 / t34 / t32;
        let t40 = 0.48827323e1 + 0.146297e-1 * t31 * sigma0 * t36;
        let t43 = 0.58827323e1 - 0.2384107471346329e2 / t40;
        let t44 = t25 * t43;
        let t45 = t26 * t26;
        let t47 = 3.0 / 10.0 * t45 * t29;
        let t49 = 1.0 / t34 / rho0;
        let t50 = tau0 * t49;
        let t51 = t47 - t50;
        let t52 = t47 + t50;
        let t53 = 1.0 / t52;
        let t56 = t51 * t51;
        let t57 = t52 * t52;
        let t58 = 1.0 / t57;
        let t61 = t56 * t51;
        let t62 = t57 * t52;
        let t63 = 1.0 / t62;
        let t66 = t56 * t56;
        let t67 = t57 * t57;
        let t68 = 1.0 / t67;
        let t71 = 1.0 - 0.1637571e0 * t51 * t53 - 0.1880028e0 * t56 * t58 - 0.4490609e0 * t61 * t63 - 0.82359e-2 * t66 * t68;
        let t72 = t44 * t71;
        let t75 = piecewise3(t2, 0.0, -0.98727272578809758046e-1 * t24 * t72);
        let t76 = rho1 <= dens_threshold;
        let t77 = -t14;
        let t79 = piecewise5(t12, t9, t8, t13, t77 * t5);
        let t80 = 1.0 + t79;
        let t81 = t80 <= zeta_threshold;
        let t82 = pow_1_3(t80);
        let t84 = piecewise3(t81, t20, t82 * t80);
        let t85 = t3 * t84;
        let t86 = rho1 * rho1;
        let t87 = pow_1_3(rho1);
        let t88 = t87 * t87;
        let t90 = 1.0 / t88 / t86;
        let t94 = 0.48827323e1 + 0.146297e-1 * t31 * sigma2 * t90;
        let t97 = 0.58827323e1 - 0.2384107471346329e2 / t94;
        let t98 = t25 * t97;
        let t100 = 1.0 / t88 / rho1;
        let t101 = tau1 * t100;
        let t102 = t47 - t101;
        let t103 = t47 + t101;
        let t104 = 1.0 / t103;
        let t107 = t102 * t102;
        let t108 = t103 * t103;
        let t109 = 1.0 / t108;
        let t112 = t107 * t102;
        let t113 = t108 * t103;
        let t114 = 1.0 / t113;
        let t117 = t107 * t107;
        let t118 = t108 * t108;
        let t119 = 1.0 / t118;
        let t122 = 1.0 - 0.1637571e0 * t102 * t104 - 0.1880028e0 * t107 * t109 - 0.4490609e0 * t112 * t114 - 0.82359e-2 * t117 * t119;
        let t123 = t98 * t122;
        let t126 = piecewise3(t76, 0.0, -0.98727272578809758046e-1 * t85 * t123);
        let tzk0 = t75 + t126;
        zk[ip] += tzk0;
    }
}
