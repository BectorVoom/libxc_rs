//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2463/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2463(t11670: f64, t11772: f64, t3114: f64, t11773: f64, t11926: f64, t11858: f64, t15688: f64, t1020: f64, t12003: f64, t12077: f64, t15905: f64, t994: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43065 = t11670 * t11772;
    let t43066 = t3114 * t43065;
    let t43069 = t11926 * t11773;
    let t43082 = t11858 * t15688;
    let t43091 = t1020 * t12003;
    let t43105 = t994 * t12077 * t15905;
    (t43065, t43066, t43069, t43082, t43091, t43105)
}
