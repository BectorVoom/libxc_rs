//! MGGA_X_2D_PRHG07 exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_2d_prhg07.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI, M_SQRT2};
use libxc_kernel_math::lambert_w::{lambert_w};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_2d_prhg07_exc_pol(
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
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t7 = 2.0 * rho0 * t4 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t11 = 2.0 * rho1 * t4 <= zeta_threshold;
        let t12 = -t8;
        let t13 = rho0 - rho1;
        let t15 = piecewise5(t7, t8, t11, t12, t13 * t4);
        let t16 = 1.0 + t15;
        let t17 = t16 <= zeta_threshold;
        let t18 = f64::sqrt(zeta_threshold);
        let t19 = t18 * zeta_threshold;
        let t20 = f64::sqrt(t16);
        let t21 = t20 * t16;
        let t22 = piecewise3(t17, t19, t21);
        let t23 = M_PI * t22;
        let t24 = M_SQRT2;
        let t25 = f64::sqrt(t3);
        let t26 = t24 * t25;
        let t27 = rho0 * rho0;
        let t28 = 1.0 / t27;
        let t32 = t27 * rho0;
        let t33 = 1.0 / t32;
        let t37 = 1.0 / M_PI;
        let t38 = (lapl0 * t28 / 4.0 - tau0 * t28 + sigma0 * t33 / 8.0) * t37;
        let t39 = -0.9999999999e0 < t38;
        let t40 = piecewise3(t39, t38, -0.9999999999e0);
        let t41 = f64::exp(-1.0);
        let t43 = lambert_w(t40 * t41);
        let t44 = t43 + 1.0;
        let t45 = t44 / 2.0;
        let t46 = xc_bessel_I0(t45);
        let t47 = t26 * t46;
        let t50 = piecewise3(t2, 0.0, -t23 * t47 / 8.0);
        let t51 = rho1 <= dens_threshold;
        let t52 = -t13;
        let t54 = piecewise5(t11, t8, t7, t12, t52 * t4);
        let t55 = 1.0 + t54;
        let t56 = t55 <= zeta_threshold;
        let t57 = f64::sqrt(t55);
        let t58 = t57 * t55;
        let t59 = piecewise3(t56, t19, t58);
        let t60 = M_PI * t59;
        let t61 = rho1 * rho1;
        let t62 = 1.0 / t61;
        let t66 = t61 * rho1;
        let t67 = 1.0 / t66;
        let t71 = (lapl1 * t62 / 4.0 - tau1 * t62 + sigma2 * t67 / 8.0) * t37;
        let t72 = -0.9999999999e0 < t71;
        let t73 = piecewise3(t72, t71, -0.9999999999e0);
        let t75 = lambert_w(t73 * t41);
        let t76 = t75 + 1.0;
        let t77 = t76 / 2.0;
        let t78 = xc_bessel_I0(t77);
        let t79 = t26 * t78;
        let t82 = piecewise3(t51, 0.0, -t60 * t79 / 8.0);
        let tzk0 = t50 + t82;
        zk[ip] += tzk0;
    }
}
