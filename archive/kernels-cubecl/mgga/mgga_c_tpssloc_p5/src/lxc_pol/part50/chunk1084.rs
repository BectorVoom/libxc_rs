//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1084/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1084<F: Float>(t31170: F, t32721: F, t1831: F, t8466: F, t31154: F, t31161: F, t31178: F, t32712: F, t32715: F, t32718: F, t539: F, t31137: F, t7691: F) -> (F, F, F) {
    let t32722 = t31170 * t32721;
    let t32724 = t8466 * t1831;
    let t32726 = -t31154 - F::cast_from(0.48447307312968469025e-2_f64) * t32712 - t31161 - F::cast_from(0.80745512188280781708e-3_f64) * t32715 + t32718 / F::cast_from(1536.0_f64) - t32722 / F::cast_from(1536.0_f64) - t31178 - t32724 / F::cast_from(384.0_f64);
    let t32727 = t539 * t32726;
    let t32731 = t31137 * t7691;
    (t32726, t32727, t32731)
}
