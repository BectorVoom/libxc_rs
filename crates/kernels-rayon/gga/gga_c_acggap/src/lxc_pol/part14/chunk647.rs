//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 647/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk647(t1524: f64, t540: f64, t960: f64, t1165: f64, t1439: f64, t4267: f64, t1181: f64, t1454: f64, t1533: f64, t5862: f64, t1761: f64, t3409: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6319 = t540 * t1524;
    let t6320 = t960 * t6319;
    let t6324 = t1165 * t4267 * t1439;
    let t6328 = t1181 * t4267 * t1454;
    let t6332 = t1165 * t5862 * t1533;
    let t6335 = t3409 * t1761;
    (t6319, t6320, t6324, t6328, t6332, t6335)
}
