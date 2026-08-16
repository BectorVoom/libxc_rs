//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 398/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk398(t150: f64, t1603: f64, t187: f64, t119: f64, t545: f64, t557: f64, t857: f64, t322: f64, t556: f64, t449: f64) -> (f64, f64, f64, f64) {
    let t1605 = t1603 * t150 * t187;
    let t1608 = t119 * t545;
    let t1611 = t857 * t557;
    let t1613 = t556 * t322;
    let t1614 = t449 * t1613;
    (t1605, t1608, t1611, t1614)
}
