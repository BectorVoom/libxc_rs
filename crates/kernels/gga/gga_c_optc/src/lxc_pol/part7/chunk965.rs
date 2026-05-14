//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 965/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk965<F: Float>(t6560: F, t6802: F, t2024: F, t22246: F, t105: F, t635: F, t6990: F, t6879: F, t136: F, t634: F, t6922: F, t648: F, t2074: F, t6893: F, t616: F, t2067: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22777 = t6802 * t6560;
    let t22781 = t22246 * t2024;
    let t22786 = t105 * t6990 * t635;
    let t22787 = t2024 * t2024;
    let t22788 = t22246 * t22787;
    let t22792 = t22246 * t6879;
    let t22797 = t634 * t6922 * t136;
    let t22798 = t22797 * t648;
    let t22800 = t6893 * t2074;
    let t22806 = t2024 * t616;
    let t22807 = t22806 * t2067;
    (t22777, t22781, t22786, t22787, t22788, t22792, t22798, t22800, t22807)
}
