//! GGA_C_LM exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_lm.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_lm_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_lm_f: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = 1.0 / M_PI;
        let t2 = 1.0 / rho[ip];
        let t5 = 1.0 + t1 * t2 / 36000.0;
        let t6 = M_CBRT3;
        let t7 = t6 * t6;
        let t8 = pow_1_3::<f64>(t1);
        let t9 = 1.0 / t8;
        let t10 = t7 * t9;
        let t11 = M_CBRT4;
        let t12 = pow_1_3::<f64>(rho[ip]);
        let t14 = t10 * t11 * t12;
        let t16 = 1.0 + 10.0 * t14;
        let t17 = f64::ln(t16);
        let t19 = 0.252e-1 * t5 * t17;
        let t20 = t8 * t8;
        let t21 = t7 * t20;
        let t22 = t12 * t12;
        let t23 = 1.0 / t22;
        let t24 = t11 * t23;
        let t25 = t21 * t24;
        let t26 = 0.7e-5 * t25;
        let t27 = t6 * t8;
        let t28 = t11 * t11;
        let t31 = t27 * t28 / t12;
        let t32 = 0.105e-3 * t31;
        let t33 = 1.0 <= zeta_threshold;
        let t34 = pow_1_3::<f64>(zeta_threshold);
        let t36 = piecewise3::<f64>(t33, t34 * zeta_threshold, 1.0);
        let t39 = M_CBRT2;
        let t43 = (2.0 * t36 - 2.0) / (2.0 * t39 - 2.0);
        let t45 = 1.0 + 0.56588424210451674939e-6 * t2;
        let t47 = 1.0 + 25.0 * t14;
        let t48 = f64::ln(t47);
        let t54 = t43 * (-0.127e-1 * t45 * t48 - 0.64355555555555555556e-5 * t25 + 0.83833333333333333334e-4 * t31 - 0.41666666666666666667e-2 + t19);
        let t55 = M_PI * t7;
        let t56 = M_PI * M_PI;
        let t57 = pow_1_3::<f64>(t56);
        let t59 = 1.0 / t57 / t56;
        let t60 = rho[ip] * rho[ip];
        let t62 = 1.0 / t22 / t60;
        let t63 = sigma[ip] * t62;
        let t66 = t34 * t34;
        let t68 = piecewise3::<f64>(t33, t66 * zeta_threshold, 1.0);
        let t69 = f64::sqrt(t68);
        let t70 = 1.0 / t69;
        let t72 = f64::powf(t1, 1.0 / 6.0);
        let t73 = 1.0 / t72;
        let t74 = f64::sqrt(sigma[ip]);
        let t75 = t73 * t74;
        let t76 = f64::powf(rho[ip], 1.0 / 6.0);
        let t81 = f64::exp(-t6 * param_lm_f * t75 / t76 / rho[ip]);
        let t82 = t70 * t81;
        let t86 = t59 * (-7.0 / 9.0 * t63 * t36 + 2.0 * t82 * t63);
        let t89 = t55 * t86 * t12 / 144.0;
        let tzk0 = -t19 + t26 - t32 + 0.84e-2 + t54 + t89;
        zk[ip] += tzk0;
    }
}
