//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 647/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk647<F: Float>(t5289: F, t7307: F, t5320: F, t739: F, t5330: F, t79: F, t6707: F) -> (F, F, F, F) {
    let t7308 = t5289 * t7307;
    let t7310 = t739 * t5320;
    let t7311 = t79 * t5330;
    let t7312 = t7311 * t6707;
    (t7308, t7310, t7311, t7312)
}
