//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1145/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1145(t33953: f64, t5275: f64, t13287: f64, t31195: f64, t2299: f64, t7630: f64, t1413: f64, t7712: f64, t2310: f64, t2001: f64, t4728: f64, t5270: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36323 = t33953 * t5275;
    let t36325 = t31195 * t13287 * t36323;
    let t36327 = t7630 * t2299;
    let t36331 = t7712 * t1413;
    let t36333 = t7630 * t2310;
    let t36335 = t2001 * t4728;
    let t36344 = t31195 * t13287 * t33953 * t5270;
    (t36323, t36325, t36327, t36331, t36333, t36335, t36344)
}
