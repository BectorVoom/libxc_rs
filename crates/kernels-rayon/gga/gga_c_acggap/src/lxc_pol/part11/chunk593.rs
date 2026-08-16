//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 593/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk593(t506: f64, t864: f64, t368: f64, t398: f64, t1036: f64, t171: f64, t3221: f64, t495: f64, t922: f64, t175: f64, t1089: f64, t1165: f64, t1552: f64, t3176: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4347 = t506 * t864;
    let t4349 = t398 * t368 * t4347;
    let t4350 = t1036 * t4349;
    let t4352 = t3221 * t171;
    let t4353 = t495 * t922;
    let t4355 = t4352 * t175 * t4353;
    let t4358 = t495 * t864;
    let t4360 = t1089 * t175 * t4358;
    let t4361 = t1036 * t4360;
    let t4364 = t1165 * t1552 * t3176;
    (t4347, t4349, t4350, t4352, t4353, t4355, t4358, t4360, t4361, t4364)
}
