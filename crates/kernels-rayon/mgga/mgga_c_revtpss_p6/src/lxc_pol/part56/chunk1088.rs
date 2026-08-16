//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1088/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1088(t28063: f64, t8634: f64, t2014: f64, t28176: f64, t32098: f64, t28043: f64, t28019: f64, t4147: f64, t2034: f64, t33594: f64, t7235: f64, t5591: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t125409 = 4.0_f64 * t8634 * t28063;
    let t125415 = 3.0_f64 * t2014 * t32098 * t28176;
    let t125417 = 4.0_f64 * t8634 * t28043;
    let t125428 = t4147 * t28019;
    let t125431 = 2.0_f64 * t2014 * t2034 * t125428;
    let t125432 = t7235 * t33594;
    let t125453 = t4147 * t5591;
    (t125409, t125415, t125417, t125431, t125432, t125453)
}
