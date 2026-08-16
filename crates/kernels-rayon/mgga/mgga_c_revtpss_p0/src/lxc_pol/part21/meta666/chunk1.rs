//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2465/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2465(t225: f64, t42059: f64, t11675: f64, t11711: f64, t11666: f64, t11710: f64, t4899: f64, t11262: f64, t3127: f64, t3129: f64, t11630: f64, t11633: f64, t3172: f64) -> (f64, f64, f64, f64, f64) {
    let t43154 = t42059 * t225;
    let t43169 = t11675 * t11711;
    let t43172 = t4899 * t11710 * t11666;
    let t43204 = t3127 * t11262 * t3129;
    let t43211 = t11630 * t3172 * t11633;
    (t43154, t43169, t43172, t43204, t43211)
}
