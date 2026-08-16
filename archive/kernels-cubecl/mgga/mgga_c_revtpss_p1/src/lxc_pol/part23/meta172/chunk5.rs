//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1036/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1036<F: Float>(t4893: F, t4894: F, t3117: F, t3317: F, t4891: F) -> (F, F, F) {
    let t4895 = t4893 * t4894;
    let t4896 = t3117 * t4895;
    let t4899 = t3317 * t4891;
    (t4895, t4896, t4899)
}
