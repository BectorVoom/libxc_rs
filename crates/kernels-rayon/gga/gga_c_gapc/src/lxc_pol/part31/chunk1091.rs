//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1091/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1091(t2404: f64, t2546: f64, t122: f64, t188: f64, t311: f64, t6851: f64, t6: f64, t6856: f64, t2387: f64, t2577: f64, t2598: f64, t2299: f64, t286: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19055 = t2546 * t2404;
    let t19094 = t311 * t6851 * t122 * t188;
    let t19097 = t6856 * t6;
    let t19120 = t2387 * t2577;
    let t19139 = t2598 * t2404;
    let t19159 = t2299 * t286;
    (t19055, t19094, t19097, t19120, t19139, t19159)
}
