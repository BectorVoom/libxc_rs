//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1010/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1010(t6: f64, t6856: f64, t2387: f64, t2577: f64, t2404: f64, t2598: f64, t2299: f64, t286: f64, t442: f64, t8139: f64, t2642: f64, t2763: f64, t2766: f64) -> (f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t19097 = t6856 * t6;
    let t19120 = t2387 * t2577;
    let t19139 = t2598 * t2404;
    let t19159 = t2299 * t286;
    let t19161 = t8139 * t19159 * t442;
    let t19179 = pi * t2642 * t2763 * t2766;
    (t19097, t19120, t19139, t19159, t19161, t19179)
}
