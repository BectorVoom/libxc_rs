//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1203/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1203(t40808: f64, t2655: f64, t9912: f64, t2745: f64, t2528: f64, t9716: f64, t193: f64, t202: f64, t2752: f64, t39549: f64, t39563: f64, t40793: f64, t40795: f64, t40797: f64, t40799: f64, t40801: f64, t40803: f64, t40805: f64, t40807: f64) -> (f64, f64, f64, f64) {
    let t40809 = 48.0_f64 * t40808;
    let t40811 = 24.0_f64 * t9912 * t2655;
    let t40812 = t2745 * t2745;
    let t40817 = t9716 * t2528;
    let t40818 = 0.10389515463408878255e3_f64 * t40817;
    let t40819 = -3.0_f64 * t193 * t202 * t2752 * t40812 + t39549 + t39563 + t40793 + t40795 + t40797 + t40799 + t40801 - t40803 - t40805 + t40807 + t40809 + t40811 - t40818;
    (t40809, t40811, t40818, t40819)
}
