//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1011/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1011(t2315: f64, t7389: f64, t672: f64, t818: f64, t1087: f64, t2299: f64, t1908: f64, t3140: f64, t198: f64, t5698: f64, t203: f64, t19: f64, t5700: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19196 = t7389 * t2315;
    let t19204 = t672 * t818;
    let t19210 = t1087 * t2299;
    let t19422 = t3140 * t1908;
    let t19507 = t198 * t5698;
    let t19508 = t19507 * t203;
    let t19509 = t5700 * t19;
    (t19196, t19204, t19210, t19422, t19507, t19508, t19509)
}
