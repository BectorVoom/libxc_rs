//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 564/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk564<F: Float>(t605: F, t6718: F, t144: F, t1901: F, t28: F, t446: F, t5854: F, t5880: F, t5941: F, t6627: F, t6632: F, t6636: F, t6641: F, t6645: F, t6649: F, t6653: F, t6687: F, t6692: F, t6696: F, t6701: F, t6705: F, t6710: F, t89: F) -> (F, F, F) {
    let t6719 = t605 * t6718;
    let t6720 = t144 * t6719;
    let t6723 = t5854 + t1901 * t6627 / 9.0 + 2.0 / 3.0 * t446 * t6632 - t446 * t6636 / 3.0 + t446 * t6641 / 3.0 - t446 * t6645 / 3.0 - t5880 - t446 * t6649 / 9.0 - t446 * t6653 / 3.0 + t89 * t28 * t6687 / 3.0 - t446 * t6692 / 3.0 + t5941 + t1901 * t6696 / 9.0 + t446 * t6701 / 3.0 - t446 * t6705 / 3.0 + 2.0 / 3.0 * t446 * t6710 - t446 * t6720 / 3.0;
    (t6719, t6720, t6723)
}
