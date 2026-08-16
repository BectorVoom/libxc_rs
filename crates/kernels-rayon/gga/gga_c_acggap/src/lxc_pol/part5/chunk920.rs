//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 920/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk920(t14047: f64, t3363: f64, t1089: f64, t175: f64, t301: f64, t3037: f64, t3210: f64, t360: f64, t368: f64, t398: f64, t1095: f64, t372: f64) -> (f64, f64, f64, f64) {
    let t14059 = t14047 * t3363;
    let t14072 = t3210 * t1089 * t175 * t3037 * t301;
    let t14081 = t3210 * t398 * t368 * t3037 * t360;
    let t14086 = t3210 * t398 * t1095 * t3037 * t372;
    (t14059, t14072, t14081, t14086)
}
