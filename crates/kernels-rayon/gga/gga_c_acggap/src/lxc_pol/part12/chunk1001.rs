//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1001/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1001(t2294: f64, t7630: f64, t31253: f64, t527: f64, t2299: f64, t7610: f64, t2310: f64, t7780: f64, t137: f64, t4838: f64, t1083: f64, t1089: f64, t598: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33865 = t7630 * t2294;
    let t33867 = t31253 * t527;
    let t33869 = t7610 * t2299;
    let t33874 = t7610 * t2310;
    let t33876 = t7780 * t2294;
    let t33878 = t137 * t4838;
    let t33881 = t598 * t1089 * t1083 * t33878;
    (t33865, t33867, t33869, t33874, t33876, t33878, t33881)
}
