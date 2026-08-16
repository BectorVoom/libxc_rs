//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 914/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk914(t8639: f64, t8589: f64, t8591: f64, t8593: f64, t8603: f64, t8606: f64, t8609: f64, t8622: f64, t8625: f64, t8657: f64, t8660: f64, t1045: f64) -> (f64, f64) {
    let t8662 = 28.0_f64 / 27.0_f64 * t8639;
    let t8673 = -t8662 - 4.0_f64 / 9.0_f64 * t8589 + 2.0_f64 / 9.0_f64 * t8593 - 2.0_f64 / 3.0_f64 * t8603 + t8591 / 3.0_f64 - 10.0_f64 / 27.0_f64 * t8622 + 4.0_f64 / 3.0_f64 * t8606 - 2.0_f64 / 3.0_f64 * t8657 - 2.0_f64 * t8609 + 2.0_f64 * t8660 - t8625 / 3.0_f64;
    let t8674 = t1045 * t8673;
    (t8673, t8674)
}
