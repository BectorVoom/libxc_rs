//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 539/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk539(t1461: f64, t3137: f64, t1030: f64, t144: f64, t674: f64) -> (f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t3138 = t1461 * t3137;
    let t3139 = t1030 * t3138;
    let t3140 = pi * t144;
    let t3141 = t3140 * t674;
    (t3138, t3139, t3140, t3141)
}
