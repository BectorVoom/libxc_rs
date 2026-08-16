//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 907/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk907(t2087: f64, t7780: f64, t3806: f64, t7741: f64, t3055: f64, t597: f64, t7670: f64, t1998: f64, t3811: f64, t7528: f64, t7799: f64, t2117: f64, t980: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31224 = t7780 * t2087;
    let t31226 = t7741 * t3806;
    let t31227 = 0.25724410870841842183e-2_f64 * t31226;
    let t31228 = t3055 * t597;
    let t31229 = t31228 * t7670;
    let t31230 = 0.64311027177104605458e-3_f64 * t31229;
    let t31231 = t1998 * t3811;
    let t31241 = t7799 * t7528;
    let t31253 = t980 * t2117;
    (t31224, t31227, t31228, t31230, t31231, t31241, t31253)
}
