//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2759/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2759<F: Float>(t46917: F, t6871: F, t22298: F, t48862: F, t48863: F, t22098: F, t9962: F, t22102: F, t46740: F, t22299: F, t22295: F, t22111: F) -> (F, F, F, F, F, F, F) {
    let t73778 = t46917 * t6871;
    let t73781 = t48862 * t48863 * t22298;
    let t73787 = t9962 * t22098;
    let t73789 = t46740 * t22102;
    let t73798 = t9962 * t22299;
    let t73800 = t9962 * t22295;
    let t73803 = t9962 * t22111;
    (t73778, t73781, t73787, t73789, t73798, t73800, t73803)
}
