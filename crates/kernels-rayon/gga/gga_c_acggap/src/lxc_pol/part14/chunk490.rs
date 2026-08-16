//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 490/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk490(t203: f64, t328: f64, t84: f64, t281: f64, t6: f64) -> (f64, f64) {
    let t2604 = t203 * t328 * t84;
    let t2605 = t281 * t2604;
    let t2606 = 0.56968947174242584612e-3_f64 * t2605;
    let t2607 = t6 * t328;
    (t2606, t2607)
}
