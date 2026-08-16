//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1534/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1534(t3204: f64, t3230: f64, t225: f64, t42059: f64, t366: f64, t1053: f64, t11940: f64, t11675: f64, t11711: f64, t11666: f64, t11710: f64, t4899: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43151 = t3204 * t3230;
    let t43154 = t42059 * t225;
    let t43155 = t43154 * t366;
    let t43161 = t11940 * t1053;
    let t43169 = t11675 * t11711;
    let t43172 = t4899 * t11710 * t11666;
    (t43151, t43154, t43155, t43161, t43169, t43172)
}
