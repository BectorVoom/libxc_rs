//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 670/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk670<F: Float>(t1121: F, t5047: F, t7748: F, t1176: F, t374: F, t283: F) -> (F, F, F, F) {
    let t7749 = t5047 * t1121;
    let t7750 = t7748 * t7749;
    let t7752 = t374 * t1176;
    let t7754 = t374 * t283;
    (t7749, t7750, t7752, t7754)
}
