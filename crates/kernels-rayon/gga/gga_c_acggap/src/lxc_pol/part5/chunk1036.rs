//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1036/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1036(t1140: f64, t4791: f64, t3409: f64, t4300: f64, t1165: f64, t12935: f64, t3355: f64, t3402: f64, t530: f64, t4713: f64, t13084: f64, t4921: f64) -> (f64, f64, f64, f64, f64) {
    let t17811 = t1140 * t4791;
    let t17821 = t3409 * t4300;
    let t17826 = t12935 * t3402 * t1165 * t530 * t3355;
    let t17831 = t3409 * t4713;
    let t17837 = t13084 * t4921;
    (t17811, t17821, t17826, t17831, t17837)
}
