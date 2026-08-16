//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1048/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1048(t6560: f64, t6802: f64, t2024: f64, t22246: f64, t105: f64, t635: f64, t6990: f64, t6879: f64, t136: f64, t634: f64, t6922: f64, t648: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22777 = t6802 * t6560;
    let t22781 = t22246 * t2024;
    let t22786 = t105 * t6990 * t635;
    let t22787 = t2024 * t2024;
    let t22788 = t22246 * t22787;
    let t22792 = t22246 * t6879;
    let t22797 = t634 * t6922 * t136;
    let t22798 = t22797 * t648;
    (t22777, t22781, t22786, t22787, t22788, t22792, t22798)
}
