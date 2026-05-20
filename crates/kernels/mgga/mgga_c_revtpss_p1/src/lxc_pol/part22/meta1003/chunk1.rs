//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3419/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3419<F: Float>(t52035: F, t52037: F, t52039: F, t52041: F, t52045: F, t52047: F, t52049: F, t52051: F, t52065: F, t63393: F, t63396: F, t63399: F, t63469: F, t63471: F) -> F {
    let t64261 = F::cast_from(0.18363555555555555555e1_f64) * t52035 - F::cast_from(0.6121185185185185185e0_f64) * t52037 - F::cast_from(0.13772666666666666666e1_f64) * t52039 - F::cast_from(0.68863333333333333332e0_f64) * t52041 - F::cast_from(0.13772666666666666666e1_f64) * t52045 + F::cast_from(0.45908888888888888888e0_f64) * t52047 + F::cast_from(0.22954444444444444444e0_f64) * t52049 + F::cast_from(0.38257407407407407407e0_f64) * t52051 + F::cast_from(0.13892666666666666667e0_f64) * t52065 - F::cast_from(0.18523555555555555556e0_f64) * t63393 + F::new(0.6311625e0) * t63396 - F::new(0.123954e2) * t63399 + F::new(0.6311625e0) * t63469 + F::cast_from(0.264729375e1_f64) * t63471;
    t64261
}
