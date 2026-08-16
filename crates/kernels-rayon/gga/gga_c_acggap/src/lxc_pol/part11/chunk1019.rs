//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1019/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1019(t4713: f64, t7822: f64, t7637: f64, t8506: f64, t137: f64, t4099: f64, t1426: f64, t368: f64, t598: f64, t4806: f64, t1980: f64, t7476: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34041 = t7822 * t4713;
    let t34043 = t7637 * t8506;
    let t34045 = t137 * t4099;
    let t34048 = t598 * t1426 * t368 * t34045;
    let t34050 = t368 * t4806;
    let t34052 = t1980 * t7476 * t34050;
    (t34041, t34043, t34045, t34048, t34050, t34052)
}
