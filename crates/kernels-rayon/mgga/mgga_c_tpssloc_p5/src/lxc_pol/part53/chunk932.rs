//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 932/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk932(t111: f64, t7222: f64, t25: f64, t40772: f64, t1519: f64, t213: f64, t225: f64, t794: f64, t214: f64, t4265: f64, t28: f64, t1834: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t84033 = t7222 * t111;
    let t86716 = t40772 * t25;
    let t86873 = t213 * t1519 * t225;
    let t86893 = t794 * t1519;
    let t87782 = t214 * t4265;
    let t89953 = t40772 * t28;
    let t90544 = t794 * t1834;
    (t84033, t86716, t86873, t86893, t87782, t89953, t90544)
}
