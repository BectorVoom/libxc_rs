//! GGA_X_2D_B86 vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_2d_b86.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_2d_b86_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = f64::sqrt(M_PI);
        let t5 = 1.0 <= zeta_threshold;
        let t6 = zeta_threshold - 1.0;
        let t8 = piecewise5(t5, t6, t5, -t6, 0.0);
        let t9 = 1.0 + t8;
        let t11 = f64::sqrt(zeta_threshold);
        let t13 = f64::sqrt(t9);
        let t15 = piecewise3(t9 <= zeta_threshold, t11 * zeta_threshold, t13 * t9);
        let t17 = M_SQRT2;
        let t18 = 1.0 / t3 * t15 * t17;
        let t19 = f64::sqrt(rho[ip]);
        let t20 = rho[ip] * rho[ip];
        let t21 = t20 * rho[ip];
        let t23 = sigma[ip] / t21;
        let t25 = 1.0 + 0.421e-2 * t23;
        let t28 = 1.0 + 0.238e-3 * t23;
        let t29 = 1.0 / t28;
        let t33 = piecewise3(t2, 0.0, -2.0 / 3.0 * t18 * t19 * t25 * t29);
        let tzk0 = 2.0 * t33;
        zk[ip] += tzk0;
        let t39 = t15 * t17;
        let t41 = 1.0 / t19 / t21;
        let t47 = t28 * t28;
        let t48 = 1.0 / t47;
        let t50 = t25 * t48 * sigma[ip];
        let t54 = piecewise3(t2, 0.0, -t18 / t19 * t25 * t29 / 3.0 + 0.47504762934721079361e-2 * t39 * t41 * sigma[ip] * t29 - 0.26855424176873199259e-3 * t39 * t41 * t50);
        let tvrho0 = 2.0 * rho[ip] * t54 + 2.0 * t33;
        vrho[ip] += tvrho0;
        let t58 = 1.0 / t19 / t20;
        let t62 = t58 * t25;
        let t67 = piecewise3(t2, 0.0, -0.15834920978240359787e-2 * t39 * t58 * t29 + 0.8951808058957733086e-4 * t39 * t62 * t48);
        let tvsigma0 = 2.0 * rho[ip] * t67;
        vsigma[ip] += tvsigma0;
    }
}
