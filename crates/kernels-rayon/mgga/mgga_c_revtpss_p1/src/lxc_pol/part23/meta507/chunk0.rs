//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2000/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2000(t17661: f64, t5401: f64, t1214: f64, t1715: f64, t1250: f64, t17353: f64, t5052: f64, t5406: f64, t1794: f64, t3617: f64, t372: f64, t5047: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20929 = t17661 * t5401;
    let t20932 = t1715 * t1214;
    let t20933 = t1250 * t20932;
    let t20934 = t17353 * t20933;
    let t20937 = t1250 * t5052;
    let t20938 = t17353 * t20937;
    let t20941 = t17661 * t5406;
    let t20944 = t3617 * t1794;
    let t20945 = t372 * t20944;
    let t20946 = t1250 * t5047;
    (t20929, t20932, t20933, t20934, t20937, t20938, t20941, t20944, t20945, t20946)
}
