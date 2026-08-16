//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1906/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1906<F: Float>(t22635: F, t26332: F, t3734: F, t90591: F, t22916: F, t26193: F, t6888: F, t22633: F, t26354: F, t90506: F, t26211: F, t6883: F) -> (F, F, F, F) {
    let t90594 = t90591 * t22635 * t26332 * t3734;
    let t90598 = t6888 * t26193 * t22916;
    let t90602 = t22633 * t22635 * t26354 * t90506;
    let t90604 = t6883 * t26211;
    (t90594, t90598, t90602, t90604)
}
