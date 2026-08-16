//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1865/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1865<F: Float>(t13184: F, t221: F, t87420: F, t25120: F, t6604: F, t81962: F, t13196: F, t25119: F, t841: F, t13204: F, t6581: F, t7500: F, t81911: F) -> (F, F, F, F, F) {
    let t87422 = t87420 * t221 * t13184;
    let t87425 = t81962 * t6604 * t25120;
    let t87428 = t25119 * t841 * t13196;
    let t87430 = t6581 * t13204;
    let t87432 = t81911 * t7500;
    (t87422, t87425, t87428, t87430, t87432)
}
