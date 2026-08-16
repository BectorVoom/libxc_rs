//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1228/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1228(t1174: f64, t15578: f64, t1177: f64, t14749: f64, t14753: f64, t14744: f64, t1011: f64, t15031: f64, t1212: f64, t1226: f64, t4965: f64, t11652: f64, t11665: f64, t11678: f64, t11692: f64, t11699: f64, t11703: f64, t1218: f64, t1232: f64, t15560: f64, t15564: f64, t15569: f64, t15574: f64, t3496: f64, t3580: f64, t4950: f64, t5002: f64) -> f64 {
    let t15580 = t1174 * t15578 / 216.0_f64;
    let t15581 = t1177 * t14749;
    let t15584 = t1177 * t14753;
    let t15587 = t1177 * t14744;
    let t15590 = t15031 * t1011;
    let t15591 = t15590 * t1212;
    let t15594 = t4965 * t1226;
    let t15601 = -t11678 * t15560 / 2304.0_f64 + t11692 * t15564 / 4608.0_f64 + t15569 * t3580 / 432.0_f64 - t15574 - t11665 * t4950 / 2304.0_f64 - t11652 / 4608.0_f64 - t15580 - t1174 * t15581 / 72.0_f64 - t1174 * t15584 / 144.0_f64 - t1174 * t15587 / 48.0_f64 + t15591 * t1218 / 1536.0_f64 - t15594 * t1232 / 2304.0_f64 + t5002 * t3496 / 3072.0_f64 - t11699 / 3456.0_f64 + t11703 / 4608.0_f64;
    t15601
}
