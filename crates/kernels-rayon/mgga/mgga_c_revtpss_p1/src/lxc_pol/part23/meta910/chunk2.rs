//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2925/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2925(t77798: f64, t916: f64, t23510: f64, t698: f64, t23507: f64, t141: f64, t77533: f64, t930: f64, t77537: f64, t77541: f64, t77545: f64, t52127: f64, t52128: f64, t63447: f64, t63453: f64, t63459: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t77802 = t916 * t77798;
    let t77804 = t698 * t23510;
    let t77806 = t698 * t23507;
    let t77810 = t141 * t930 * t77533;
    let t77813 = t141 * t930 * t77537;
    let t77816 = t141 * t930 * t77541;
    let t77819 = t141 * t930 * t77545;
    let t77824 = 0.258925e1_f64 * t77802 - 0.33114e0_f64 * t77804 + 0.5519e-1_f64 * t77806 - t52127 + 0.73586666666666666667e0_f64 * t52128 + 0.198684e1_f64 * t77810 - 0.149013e1_f64 * t77813 + 0.49671e0_f64 * t77816 + 0.49671e0_f64 * t77819 + 0.30192500000000000001e0_f64 * t63447 - 0.26837777777777777777e0_f64 * t63453 + 0.80513333333333333334e0_f64 * t63459;
    (t77802, t77804, t77806, t77810, t77813, t77816, t77819, t77824)
}
