//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 935/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk935(t3806: f64, t7741: f64, t3055: f64, t597: f64, t7670: f64, t1998: f64, t3811: f64, t30120: f64, t7415: f64, t1988: f64, t7523: f64, t7528: f64, t7799: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31226 = t7741 * t3806;
    let t31228 = t3055 * t597;
    let t31229 = t31228 * t7670;
    let t31231 = t1998 * t3811;
    let t31237 = t30120 * t7415;
    let t31239 = t1988 * t7523;
    let t31241 = t7799 * t7528;
    (t31226, t31228, t31229, t31231, t31237, t31239, t31241)
}
