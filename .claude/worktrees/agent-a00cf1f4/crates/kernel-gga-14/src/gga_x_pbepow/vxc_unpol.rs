//! GGA_X_PBEPOW vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbepow.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_pbepow_vxc_unpol(
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
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t21 = M_PI * M_PI;
        let t22 = pow_1_3(t21);
        let t23 = t22 * t22;
        let t24 = 1.0 / t23;
        let t25 = t20 * t24;
        let t26 = t25 * sigma[ip];
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t18 * t18;
        let t32 = 1.0 / t30 / t29;
        let t33 = t28 * t32;
        let t34 = sigma[ip] * t28;
        let t38 = 0.91464571985215458336e0 * t25 * t34 * t32 + 0.804e0;
        let t39 = 1.0 / t38;
        let t40 = t33 * t39;
        let t41 = t26 * t40;
        let t42 = f64::powf(t41, 100.0);
        let t44 = 0.13344141567995010044e-3 * t42 - 1.0;
        let t45 = t33 * t44;
        let t48 = 1.0 - 0.91464571985215458336e-2 * t26 * t45;
        let t52 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t48);
        let tzk0 = 2.0 * t52;
        zk[ip] += tzk0;
        let t54 = t17 / t30;
        let t58 = t29 * rho[ip];
        let t60 = 1.0 / t30 / t58;
        let t61 = t28 * t60;
        let t62 = t61 * t44;
        let t65 = f64::powf(t41, 99.0);
        let t66 = t61 * t39;
        let t69 = t20 * t20;
        let t72 = t69 / t22 / t21;
        let t73 = sigma[ip] * sigma[ip];
        let t74 = t72 * t73;
        let t75 = t29 * t29;
        let t76 = t75 * t29;
        let t78 = 1.0 / t18 / t76;
        let t80 = t38 * t38;
        let t81 = 1.0 / t80;
        let t82 = t27 * t78 * t81;
        let t85 = -8.0 / 3.0 * t26 * t66 + 0.4878110505878157778e1 * t74 * t82;
        let t86 = t65 * t85;
        let t90 = 0.2439055252939078889e-1 * t26 * t62 - 0.12205161970267854748e-3 * t26 * t33 * t86;
        let t95 = piecewise3(t2, 0.0, -t6 * t54 * t48 / 8.0 - 3.0 / 8.0 * t6 * t19 * t90);
        let tvrho0 = 2.0 * rho[ip] * t95 + 2.0 * t52;
        vrho[ip] += tvrho0;
        let t102 = t75 * rho[ip];
        let t106 = t27 / t18 / t102 * t81;
        let t109 = t25 * t40 - 0.18292914397043091667e1 * t72 * sigma[ip] * t106;
        let t110 = t65 * t109;
        let t114 = -0.91464571985215458336e-2 * t25 * t45 - 0.12205161970267854748e-3 * t26 * t33 * t110;
        let t118 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t114);
        let tvsigma0 = 2.0 * rho[ip] * t118;
        vsigma[ip] += tvsigma0;
    }
}
