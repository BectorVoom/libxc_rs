//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1082/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1082(t2105: f64, t4668: f64, t141: f64, t4649: f64, t4631: f64, t6893: f64, t4652: f64, t4656: f64, t2080: f64, t4661: f64, t4665: f64, t22242: f64, t4626: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t38463 = t4668 * t2105;
    let t38486 = t141 * t4649;
    let t38553 = t6893 * t4631;
    let t38668 = t6893 * t4652;
    let t38671 = t6893 * t4656;
    let t38685 = t2080 * t4661;
    let t38689 = t2080 * t4665;
    let t38749 = t22242 * t4626;
    (t38463, t38486, t38553, t38668, t38671, t38685, t38689, t38749)
}
