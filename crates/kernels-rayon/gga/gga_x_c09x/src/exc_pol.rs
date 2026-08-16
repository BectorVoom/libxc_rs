//! GGA_X_C09X exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_c09x.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_c09x_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
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
        let t29 = M_PI * M_PI;
        let t30 = pow_1_3(t29);
        let t31 = t30 * t30;
        let t32 = 1.0 / t31;
        let t33 = t28 * t32;
        let t34 = rho0 * rho0;
        let t35 = pow_1_3(rho0);
        let t36 = t35 * t35;
        let t38 = 1.0 / t36 / t34;
        let t39 = sigma0 * t38;
        let t40 = t33 * t39;
        let t42 = f64::exp(-0.20125e-2 * t40);
        let t47 = f64::exp(-0.100625e-2 * t40);
        let t49 = 0.2245e1 + 0.25708333333333333333e-2 * t33 * t39 * t42 - 0.1245e1 * t47;
        let t53 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t49);
        let t54 = rho1 <= dens_threshold;
        let t55 = -t16;
        let t57 = piecewise5(t14, t11, t10, t15, t55 * t7);
        let t58 = 1.0 + t57;
        let t59 = t58 <= zeta_threshold;
        let t60 = pow_1_3(t58);
        let t62 = piecewise3(t59, t22, t60 * t58);
        let t63 = t62 * t26;
        let t64 = rho1 * rho1;
        let t65 = pow_1_3(rho1);
        let t66 = t65 * t65;
        let t68 = 1.0 / t66 / t64;
        let t69 = sigma2 * t68;
        let t70 = t33 * t69;
        let t72 = f64::exp(-0.20125e-2 * t70);
        let t77 = f64::exp(-0.100625e-2 * t70);
        let t79 = 0.2245e1 + 0.25708333333333333333e-2 * t33 * t69 * t72 - 0.1245e1 * t77;
        let t83 = piecewise3(t54, 0.0, -3.0 / 8.0 * t5 * t63 * t79);
        let tzk0 = t53 + t83;
        zk[ip] += tzk0;
    }
}
