//! GGA_X_PBETRANS vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbetrans.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_pbetrans_vxc_unpol(
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
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5::<f64>(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3::<f64>(zeta_threshold);
        let t15 = pow_1_3::<f64>(t11);
        let t17 = piecewise3::<f64>(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3::<f64>(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_PI * M_PI;
        let t21 = pow_1_3::<f64>(t20);
        let t23 = M_CBRT6;
        let t24 = t23 * t23;
        let t27 = f64::sqrt(sigma[ip]);
        let t28 = M_CBRT2;
        let t29 = t27 * t28;
        let t31 = 1.0 / t18 / rho[ip];
        let t38 = f64::exp(-2.0 * t3 * t21 * (t24 / t21 * t29 * t31 / 12.0 - 3.0));
        let t39 = 1.0 + t38;
        let t41 = 0.413e0 / t39;
        let t42 = 0.1227e1 - t41;
        let t43 = t21 * t21;
        let t45 = t23 / t43;
        let t46 = t28 * t28;
        let t47 = sigma[ip] * t46;
        let t48 = rho[ip] * rho[ip];
        let t49 = t18 * t18;
        let t51 = 1.0 / t49 / t48;
        let t55 = 0.1227e1 - t41 + 0.91249999999999999998e-2 * t45 * t47 * t51;
        let t56 = 1.0 / t55;
        let t58 = -t42 * t56 + 1.0;
        let t60 = t42 * t58 + 1.0;
        let t64 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t60);
        let tzk0 = 2.0 * t64;
        zk[ip] += tzk0;
        let t66 = t17 / t49;
        let t70 = t39 * t39;
        let t71 = 1.0 / t70;
        let t72 = t71 * t3;
        let t73 = t24 * t27;
        let t74 = t72 * t73;
        let t76 = 1.0 / t18 / t48;
        let t77 = t28 * t76;
        let t78 = t38 * t58;
        let t79 = t77 * t78;
        let t82 = t38 * t56;
        let t83 = t77 * t82;
        let t86 = t55 * t55;
        let t87 = 1.0 / t86;
        let t88 = t42 * t87;
        let t89 = t72 * t24;
        let t90 = t76 * t38;
        let t94 = t48 * rho[ip];
        let t96 = 1.0 / t49 / t94;
        let t100 = 0.91777777777777777778e-1 * t89 * t29 * t90 - 0.24333333333333333333e-1 * t45 * t47 * t96;
        let t102 = -0.91777777777777777778e-1 * t74 * t83 + t88 * t100;
        let t104 = 0.91777777777777777778e-1 * t74 * t79 + t42 * t102;
        let t109 = piecewise3::<f64>(t2, 0.0, -t6 * t66 * t60 / 8.0 - 3.0 / 8.0 * t6 * t19 * t104);
        let tvrho0 = 2.0 * rho[ip] * t109 + 2.0 * t64;
        vrho[ip] += tvrho0;
        let t112 = 1.0 / t27;
        let t113 = t24 * t112;
        let t114 = t72 * t113;
        let t115 = t28 * t31;
        let t116 = t115 * t78;
        let t119 = t115 * t82;
        let t122 = t112 * t28;
        let t123 = t31 * t38;
        let t127 = t46 * t51;
        let t130 = -0.34416666666666666667e-1 * t89 * t122 * t123 + 0.91249999999999999998e-2 * t45 * t127;
        let t132 = 0.34416666666666666667e-1 * t114 * t119 + t88 * t130;
        let t134 = -0.34416666666666666667e-1 * t114 * t116 + t42 * t132;
        let t138 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t134);
        let tvsigma0 = 2.0 * rho[ip] * t138;
        vsigma[ip] += tvsigma0;
    }
}
