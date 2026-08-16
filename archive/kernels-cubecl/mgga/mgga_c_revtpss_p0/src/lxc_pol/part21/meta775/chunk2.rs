//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2760/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2760<F: Float>(t39419: F, t39422: F, t39429: F, t39432: F, t39442: F, t49865: F, t49867: F, t49868: F, t49869: F, t49870: F, t49872: F, t49873: F, t49877: F, t49879: F, t49882: F, t49885: F, t49892: F) -> F {
    let t50844 = t49865 - t39419 - t39422 - t49867 - t49868 - t49869 - t39429 - t39432 + t49870 + t49872 + t49873 + t39442 + t49877 + t49879 + t49882 + t49885 + t49892;
    t50844
}
