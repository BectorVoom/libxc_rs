//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 887/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk887(t1009: f64, t5848: f64, t1011: f64, t1019: f64, t10422: f64, t5908: f64, t3070: f64, t225: f64, t5915: f64, t1057: f64, t5972: f64, t690: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18028 = t5848 * t1009;
    let t18029 = t18028 * t1011;
    let t18030 = t18029 * t1019;
    let t18041 = t10422 * t5908;
    let t18042 = t3070 * t18041;
    let t18074 = t5915 * t225;
    let t18086 = t18028 * t1057;
    let t18203 = t690 * t5972;
    (t18029, t18030, t18041, t18042, t18074, t18086, t18203)
}
