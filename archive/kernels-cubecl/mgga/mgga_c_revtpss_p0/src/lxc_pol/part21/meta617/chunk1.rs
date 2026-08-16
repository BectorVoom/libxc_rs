//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2371/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2371<F: Float>(t231: F, t2782: F, t2783: F, t39709: F, t10910: F, t233: F, t689: F, t869: F, t2778: F, t39515: F, t39501: F, t871: F) -> (F, F, F, F) {
    let t40307 = t2782 * t2783 * t39709 * t231;
    let t40311 = t689 * t869 * t233 * t10910;
    let t40314 = F::cast_from(0.11564373972601816912e-1_f64) * t39515 * t2778;
    let t40316 = F::cast_from(0.56911289235245161963e-1_f64) * t39501 * t871;
    (t40307, t40311, t40314, t40316)
}
