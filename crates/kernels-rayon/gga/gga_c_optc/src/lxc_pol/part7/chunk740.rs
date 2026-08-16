//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 740/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk740(t2037: f64, t7110: f64, t2126: f64, t6781: f64, t151: f64, t6777: f64, t6791: f64, t2123: f64, t624: f64) -> (f64, f64, f64, f64, f64) {
    let t7111 = t7110 * t2037;
    let t7113 = t2126 * t6781;
    let t7116 = t151 * t6777;
    let t7119 = t151 * t6791;
    let t7122 = t2123 * t624;
    (t7111, t7113, t7116, t7119, t7122)
}
