//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 836/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk836(t435: f64, t965: f64, t1159: f64, t848: f64, t1111: f64, t301: f64, t182: f64, t862: f64, t1083: f64, t171: f64) -> (f64, f64, f64, f64, f64) {
    let t12610 = t965 * t435;
    let t12726 = t848 * t1159;
    let t12816 = t1111 * t301;
    let t12935 = t862 * t182;
    let t13287 = t171 * t1083;
    (t12610, t12726, t12816, t12935, t13287)
}
