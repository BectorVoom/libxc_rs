//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2759/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2759(t46917: f64, t6871: f64, t22298: f64, t48862: f64, t48863: f64, t22098: f64, t9962: f64, t22102: f64, t46740: f64, t22299: f64, t22295: f64, t22111: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t73778 = t46917 * t6871;
    let t73781 = t48862 * t48863 * t22298;
    let t73787 = t9962 * t22098;
    let t73789 = t46740 * t22102;
    let t73798 = t9962 * t22299;
    let t73800 = t9962 * t22295;
    let t73803 = t9962 * t22111;
    (t73778, t73781, t73787, t73789, t73798, t73800, t73803)
}
