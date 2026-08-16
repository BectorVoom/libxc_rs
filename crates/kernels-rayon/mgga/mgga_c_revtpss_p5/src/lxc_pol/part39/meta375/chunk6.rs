//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1333/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1333(t1065: f64, t4772: f64, t906: f64, t1042: f64, t2858: f64, t4823: f64, t1469: f64, t3059: f64, t4872: f64, t999: f64, t247: f64, t3116: f64) -> (f64, f64, f64, f64, f64) {
    let t16138 = t1065 * t4772;
    let t16139 = t16138 * t906;
    let t16140 = t1042 * t16139;
    let t16143 = t4823 * t2858;
    let t16144 = t1042 * t16143;
    let t16147 = t1469 * t3059;
    let t16148 = t4872 * t16147;
    let t16149 = t1042 * t16148;
    let t16152 = t4772 * t999;
    let t16154 = t247 * t3116 * t16152;
    (t16140, t16144, t16149, t16152, t16154)
}
