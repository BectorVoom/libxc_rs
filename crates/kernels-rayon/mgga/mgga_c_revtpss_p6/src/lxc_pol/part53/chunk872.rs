//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 872/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk872(t1230: f64, t7623: f64, t3636: f64, t7624: f64, t3704: f64, t7618: f64, t479: f64, t3089: f64, t1285: f64, t3717: f64, t3707: f64, t7617: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26852 = t1230 * t7623;
    let t26855 = t7624 * t3636;
    let t26863 = t7618 * t3704;
    let t26865 = sigma2 * t479;
    let t26866 = t26865 * t3089;
    let t26867 = t1285 * t26866;
    let t26870 = t3717 * t26866;
    let t26873 = t3707 * t7617;
    (t26852, t26855, t26863, t26865, t26867, t26870, t26873)
}
