//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 746/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk746<F: Float>(t6636: F, t6692: F, t6754: F, t6814: F, t6875: F, t6931: F, t6984: F, t7055: F, t481: F, t686: F, t941: F) -> (F, F) {
    let t7058 = t6636 + t6692 + t6754 + t6814 + t6875 + t6931 + t6984 + t7055;
    let t7064 = t481 * t941 * t686;
    (t7058, t7064)
}
