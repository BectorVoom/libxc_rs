//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2426/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2426(t49379: f64, t60192: f64, t60194: f64, t60202: f64, t68571: f64, t68577: f64, t68580: f64, t68583: f64, t68586: f64, t68589: f64, t68592: f64, t42212: f64, t59663: f64, t59665: f64, t59680: f64, t59688: f64, t59694: f64, t60204: f64, t68596: f64, t68599: f64, t68602: f64, t68605: f64, t68608: f64) -> (f64, f64) {
    let t69118 = -t49379 - 0.516475e0_f64 * t68571 + 0.125034e1_f64 * t60192 - 0.83356000000000000002e0_f64 * t60194 - 0.41678e0_f64 * t60202 + 0.123954e2_f64 * t68577 - 0.929655e1_f64 * t68580 + 0.309885e1_f64 * t68583 + 0.309885e1_f64 * t68586 + 0.103295e1_f64 * t68589 - 0.34431666666666666667e0_f64 * t68592;
    let t69130 = 0.68863333333333333334e1_f64 * t68596 - 0.17215833333333333334e1_f64 * t68599 + 0.61977e1_f64 * t68602 - 0.17215833333333333333e1_f64 * t68605 - 0.929655e1_f64 * t68608 - 0.11577222222222222223e0_f64 * t60204 - 0.103295e1_f64 * t59663 + 0.34431666666666666666e0_f64 * t59665 + 0.51647499999999999999e0_f64 * t59680 + 0.13772666666666666667e1_f64 * t59688 - 0.68863333333333333332e0_f64 * t59694 + t42212;
    (t69118, t69130)
}
