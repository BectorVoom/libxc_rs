//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 933/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk933(t159: f64, t793: f64, t587: f64, t65: f64, t4153: f64, t575: f64, t1455: f64, t1464: f64, t4168: f64, t571: f64, t143: f64, t2580: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7021 = t793 * t159;
    let t8779 = 1.0_f64 / t65 / t587;
    let t9263 = t4153 * t575;
    let t9265 = t1455 * t1464;
    let t9267 = t571 * t4168;
    let t9273 = 1.0_f64 / t2580 / t143;
    (t7021, t8779, t9263, t9265, t9267, t9273)
}
