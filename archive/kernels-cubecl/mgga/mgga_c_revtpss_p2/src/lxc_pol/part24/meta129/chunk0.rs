//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 681/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk681<F: Float>(t354: F, t357: F, t3298: F, t378: F) -> (F, F) {
    let t4975 = t354 * t357;
    let t4980 = t3298 * t378;
    (t4975, t4980)
}
