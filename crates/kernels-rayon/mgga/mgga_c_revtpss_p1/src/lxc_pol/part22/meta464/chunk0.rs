//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2145/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2145(t11108: f64, t1699: f64, t3022: f64, t4725: f64, t11465: f64, t1633: f64, t3015: f64, t981: f64, t3026: f64, t4719: f64, t1695: f64, t3075: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15566 = t1699 * t11108;
    let t15571 = 0.23392894490538584828e1_f64 * t3022 * t4725;
    let t15572 = t11465 * t1633;
    let t15573 = t15572 * t3015;
    let t15575 = 0.10389515463408878255e3_f64 * t981 * t15573;
    let t15577 = 0.11696447245269292414e1_f64 * t4719 * t3026;
    let t15578 = t1695 * t3075;
    (t15566, t15571, t15573, t15575, t15577, t15578)
}
