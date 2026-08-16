//! GGA_X_CAP exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_cap.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_cap_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_alphaoAx: f64,
    param_c: f64,
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
        let t29 = t28 * t28;
        let t31 = M_PI * M_PI;
        let t32 = pow_1_3(t31);
        let t33 = 1.0 / t32;
        let t34 = param_alphaoAx * t29 * t33;
        let t35 = f64::sqrt(sigma0);
        let t36 = pow_1_3(rho0);
        let t38 = 1.0 / t36 / rho0;
        let t39 = t35 * t38;
        let t40 = t29 * t33;
        let t43 = 1.0 + t40 * t39 / 12.0;
        let t44 = f64::ln(t43);
        let t46 = param_c * t44 + 1.0;
        let t47 = 1.0 / t46;
        let t48 = t44 * t47;
        let t52 = 1.0 - t34 * t39 * t48 / 12.0;
        let t56 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t52);
        let t57 = rho1 <= dens_threshold;
        let t58 = -t16;
        let t60 = piecewise5(t14, t11, t10, t15, t58 * t7);
        let t61 = 1.0 + t60;
        let t62 = t61 <= zeta_threshold;
        let t63 = pow_1_3(t61);
        let t65 = piecewise3(t62, t22, t63 * t61);
        let t66 = t65 * t26;
        let t67 = f64::sqrt(sigma2);
        let t68 = pow_1_3(rho1);
        let t70 = 1.0 / t68 / rho1;
        let t71 = t67 * t70;
        let t74 = 1.0 + t40 * t71 / 12.0;
        let t75 = f64::ln(t74);
        let t77 = param_c * t75 + 1.0;
        let t78 = 1.0 / t77;
        let t79 = t75 * t78;
        let t83 = 1.0 - t34 * t71 * t79 / 12.0;
        let t87 = piecewise3(t57, 0.0, -3.0 / 8.0 * t5 * t66 * t83);
        let tzk0 = t56 + t87;
        zk[ip] += tzk0;
    }
}
