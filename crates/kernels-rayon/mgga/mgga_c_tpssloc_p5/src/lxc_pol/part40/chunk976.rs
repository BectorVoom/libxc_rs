//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 976/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk976(t4250: f64, t9638: f64, t4240: f64, t4191: f64, t2697: f64, t4261: f64, t820: f64, t9645: f64, t1484: f64, t828: f64, t1516: f64, t9993: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13287 = 7.0_f64 / 576.0_f64 * t9638 * t4250;
    let t13320 = 7.0_f64 / 2304.0_f64 * t9638 * t4240;
    let t13330 = 7.0_f64 / 576.0_f64 * t9638 * t4191;
    let t13345 = 7.0_f64 / 576.0_f64 * t2697 * t4261;
    let t13350 = t9645 * t820;
    let t13351 = t1484 * t828;
    let t13359 = 7.0_f64 / 576.0_f64 * t9993 * t1516;
    (t13287, t13320, t13330, t13345, t13350, t13351, t13359)
}
