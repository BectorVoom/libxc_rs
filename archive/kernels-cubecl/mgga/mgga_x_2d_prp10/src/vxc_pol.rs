//! MGGA_X_2D_PRP10 vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_vxc/mgga_x_2d_prp10.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::bessel::{xc_bessel_I0, xc_bessel_I1};
use libxc_kernel_math::constants::{M_PI, M_SQRT2};
use libxc_kernel_math::lambert_w::{lambert_w};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_2d_prp10_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
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
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 * rho0;
        let t3 = 1.0 / t2;
        let t6 = tau0 * t3;
        let t8 = 1.0 / t2 / rho0;
        let t10 = sigma0 * t8 / 8.0;
        let t12 = 1.0 / M_PI;
        let t13 = (lapl0 * t3 / 4.0 - t6 + t10) * t12;
        let t14 = -0.9999999999e0 < t13;
        let t15 = piecewise3::<f64>(t14, t13, -0.9999999999e0);
        let t16 = f64::exp(-1.0);
        let t18 = lambert_w::<f64>(t15 * t16);
        let t19 = t18 + 1.0;
        let t20 = t19 / 2.0;
        let t21 = xc_bessel_I0::<f64>(t20);
        let t23 = t6 - t10;
        let t24 = 0.1e-9 < t23;
        let t25 = piecewise3::<f64>(t24, t23, 0.1e-9);
        let t26 = f64::sqrt(t25);
        let t29 = M_PI * t21 - 4.0 / 3.0 * t12 * t26;
        let t30 = f64::sqrt(rho0);
        let tvrho0 = -t29 * t30;
        vrho[ip * 2] += tvrho0;
        let t32 = rho1 * rho1;
        let t33 = 1.0 / t32;
        let t36 = tau1 * t33;
        let t38 = 1.0 / t32 / rho1;
        let t40 = sigma2 * t38 / 8.0;
        let t42 = (lapl1 * t33 / 4.0 - t36 + t40) * t12;
        let t43 = -0.9999999999e0 < t42;
        let t44 = piecewise3::<f64>(t43, t42, -0.9999999999e0);
        let t46 = lambert_w::<f64>(t44 * t16);
        let t47 = t46 + 1.0;
        let t48 = t47 / 2.0;
        let t49 = xc_bessel_I0::<f64>(t48);
        let t51 = t36 - t40;
        let t52 = 0.1e-9 < t51;
        let t53 = piecewise3::<f64>(t52, t51, 0.1e-9);
        let t54 = f64::sqrt(t53);
        let t57 = M_PI * t49 - 4.0 / 3.0 * t12 * t54;
        let t58 = f64::sqrt(rho1);
        let tvrho1 = -t57 * t58;
        vrho[ip * 2 + 1] += tvrho1;
    }
}
