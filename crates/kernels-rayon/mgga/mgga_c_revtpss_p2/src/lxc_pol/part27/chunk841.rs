//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 841/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk841(t1398: f64, t215: f64, t268: f64, t543: f64, t4101: f64, t2453: f64, t4100: f64, t281: f64, t68: f64, t10080: f64, t10082: f64, t10085: f64, t10090: f64, t10098: f64, t10102: f64, t10105: f64, t10109: f64, t10114: f64, t10117: f64, t10120: f64, t10126: f64, t10129: f64, t10130: f64, t1399: f64, t4057: f64, t4114: f64, t4118: f64, t5755: f64, t820: f64, t9912: f64, t9995: f64) -> f64 {
    let t10136 = t268 * t215 * t1398 * t543;
    let t10137 = t4101 * t10136;
    let t10139 = t2453 * t4100;
    let t10142 = t281 * t68 * t1398 * t543;
    let t10143 = t10139 * t10142;
    let t10145 = 0.32927245914677557992e-1_f64 * t10080 + 0.16463622957338778996e-1_f64 * t10085 - 0.19756347548806534796e1_f64 * t820 * t4118 * t4057 - 0.39512695097613069591e1_f64 * t820 * t10090 * t9995 + 0.39512695097613069591e1_f64 * t820 * t4114 * t9912 - 0.39029762157531132076e-1_f64 * t10098 + t10102 + 0.29272321618148349057e-1_f64 * t10105 + 0.34697458558045176417e-2_f64 * t10109 + t10114 - t10117 - 0.29272321618148349057e-1_f64 * t10120 - 0.19756347548806534796e1_f64 * t5755 * t10082 * t1399 - t10126 - t10129 - 0.19756347548806534796e1_f64 * t820 * t10130 * t1399 + 0.39029762157531132076e-1_f64 * t10137 - 0.34697458558045176417e-2_f64 * t10143;
    t10145
}
