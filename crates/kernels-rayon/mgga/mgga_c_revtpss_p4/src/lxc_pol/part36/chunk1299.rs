//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1299/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1299(t2014: f64, t22475: f64, t7934: f64, t29996: f64, t7898: f64, t30005: f64, t7732: f64, t30128: f64, t1936: f64, t25043: f64, t651: f64, t28172: f64, t29494: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t113076 = 6.0_f64 * t2014 * t7934 * t22475;
    let t113078 = 6.0_f64 * t7898 * t29996;
    let t113084 = 6.0_f64 * t7732 * t30005;
    let t113086 = 6.0_f64 * t7732 * t30128;
    let t113089 = 2.0_f64 * t651 * t25043 * t1936;
    let t113092 = 9.0_f64 * t2014 * t28172 * t29494;
    (t113076, t113078, t113084, t113086, t113089, t113092)
}
