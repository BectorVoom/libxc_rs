//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1250/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1250(t25981: f64, t820: f64, t843: f64, t4006: f64, t2681: f64, t7262: f64, t1401: f64, t7264: f64, t9901: f64, t7271: f64, t9986: f64, t9893: f64) -> (f64, f64, f64, f64, f64) {
    let t94455 = t820 * t25981 * t843;
    let t94456 = t94455 * t4006;
    let t94459 = t820 * t7262 * t2681;
    let t94460 = t94459 * t1401;
    let t94462 = t7264 * t9901;
    let t94464 = t7271 * t9986;
    let t94466 = t7264 * t9893;
    (t94456, t94460, t94462, t94464, t94466)
}
