//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 951/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk951(t2137: f64, t32123: f64, t1619: f64, t322: f64, t315: f64, t309: f64, t1219: f64, t615: f64, t8396: f64, t525: f64, t847: f64, t448: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33698 = t2137 * t32123;
    let t33699 = t1619 * t322;
    let t33743 = t315 * t32123;
    let t33744 = t1619 * t309;
    let t33778 = t615 * t8396 * t1219;
    let t33787 = t525 * t847;
    let t33795 = t8396 * t448;
    (t33698, t33699, t33743, t33744, t33778, t33787, t33795)
}
