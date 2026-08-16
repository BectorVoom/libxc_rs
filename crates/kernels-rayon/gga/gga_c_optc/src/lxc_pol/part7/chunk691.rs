//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 691/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk691(t43: f64, t1891: f64, t607: f64, t6533: f64, t6534: f64, t6537: f64, t6541: f64, t1026: f64, t52: f64, t1897: f64, t553: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t6545 = piecewise3(t44, 0.0_f64, 8.0_f64 / 27.0_f64 * t6533 * t6534 - 2.0_f64 / 3.0_f64 * t6537 * t1891 + 2.0_f64 / 3.0_f64 * t607 * t6541);
    let t6547 = 1.0_f64 / t52 / t1026;
    let t6548 = t1897 * t553;
    (t6545, t6547, t6548)
}
