//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 839/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk839(t159: f64, t619: f64, t9767: f64, t157: f64, t1838: f64, t609: f64, t2152: f64, t150: f64, t187: f64, t2331: f64, t556: f64, t2147: f64) -> (f64, f64, f64, f64, f64) {
    let t9769 = t619 * t159 * t9767;
    let t9773 = t609 * t1838 * t157;
    let t9774 = t2152 * t9773;
    let t9779 = t9767 * t150 * t187;
    let t9789 = t2331 * t556;
    let t9790 = t2147 * t9789;
    (t9769, t9774, t9779, t9789, t9790)
}
