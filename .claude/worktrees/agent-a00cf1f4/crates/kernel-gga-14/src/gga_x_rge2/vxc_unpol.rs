//! GGA_X_RGE2 vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_rge2.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_rge2_vxc_unpol(
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
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t20 = M_CBRT6;
        let t21 = M_PI * M_PI;
        let t22 = pow_1_3(t21);
        let t23 = t22 * t22;
        let t25 = t20 / t23;
        let t26 = M_CBRT2;
        let t27 = t26 * t26;
        let t28 = sigma[ip] * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t18 * t18;
        let t32 = 1.0 / t30 / t29;
        let t36 = t20 * t20;
        let t38 = 1.0 / t22 / t21;
        let t39 = t36 * t38;
        let t40 = sigma[ip] * sigma[ip];
        let t41 = t40 * t26;
        let t42 = t29 * t29;
        let t43 = t42 * rho[ip];
        let t45 = 1.0 / t18 / t43;
        let t49 = 0.804e0 + 5.0 / 972.0 * t25 * t28 * t32 + 0.65823568907145082055e-4 * t39 * t41 * t45;
        let t52 = 0.1804e1 - 0.646416e0 / t49;
        let t56 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t17 * t18 * t52);
        let tzk0 = 2.0 * t56;
        zk[ip] += tzk0;
        let t57 = 1.0 / t30;
        let t62 = t3 * t17;
        let t63 = t49 * t49;
        let t64 = 1.0 / t63;
        let t65 = t18 * t64;
        let t66 = t29 * rho[ip];
        let t68 = 1.0 / t30 / t66;
        let t72 = t42 * t29;
        let t74 = 1.0 / t18 / t72;
        let t78 = -10.0 / 729.0 * t25 * t28 * t68 - 0.35105903417144043763e-3 * t39 * t41 * t74;
        let t83 = piecewise3(t2, 0.0, -t6 * t17 * t57 * t52 / 8.0 - 0.16551095363746320496e0 * t62 * t65 * t78);
        let tvrho0 = 2.0 * rho[ip] * t83 + 2.0 * t56;
        vrho[ip] += tvrho0;
        let t89 = sigma[ip] * t26;
        let t93 = 5.0 / 972.0 * t25 * t27 * t32 + 0.13164713781429016411e-3 * t39 * t89 * t45;
        let t97 = piecewise3(t2, 0.0, -0.16551095363746320496e0 * t62 * t65 * t93);
        let tvsigma0 = 2.0 * rho[ip] * t97;
        vsigma[ip] += tvsigma0;
    }
}
