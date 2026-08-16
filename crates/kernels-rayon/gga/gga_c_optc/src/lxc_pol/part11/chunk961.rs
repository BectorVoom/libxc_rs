//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 961/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk961(t1221: f64, t17356: f64, t914: f64, t15571: f64, t4208: f64, t1102: f64, t423: f64, t5239: f64) -> (f64, f64, f64, f64, f64) {
    let t17634 = t1221 * t17356;
    let t17635 = t914 * t17634;
    let t17643 = t15571 * t4208;
    let t17645 = 0.51947267698127589897e2_f64 * t1102 * t17643;
    let t17648 = 1.0_f64 / t423 / t5239;
    (t17634, t17635, t17643, t17645, t17648)
}
