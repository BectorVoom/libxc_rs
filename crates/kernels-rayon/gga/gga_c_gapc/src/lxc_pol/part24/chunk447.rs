//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 447/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk447(t2439: f64, t2440: f64, t640: f64, t792: f64, t791: f64, t1: f64, t332: f64, t3: f64, t875: f64, t2416: f64, t126: f64, t826: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2441 = t2439 * t2440;
    let t2444 = t792 * t640;
    let t2445 = t791 * t2444;
    let t2446 = t332 * t1;
    let t2447 = t3 * t875;
    let t2448 = t2446 * t2447;
    let t2449 = t2416 * t2448;
    let t2452 = t826 * t126;
    (t2441, t2445, t2446, t2448, t2449, t2452)
}
