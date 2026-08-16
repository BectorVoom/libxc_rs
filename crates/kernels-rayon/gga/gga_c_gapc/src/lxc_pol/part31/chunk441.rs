//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 441/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk441(t2361: f64, t852: f64, t4: f64, t748: f64, t78: f64, t1365: f64, t854: f64, t106: f64, t737: f64, t2059: f64, t2078: f64, t858: f64) -> (f64, f64, f64, f64, f64) {
    let t2362 = t852 * t2361;
    let t2364 = t4 * t78 * t748;
    let t2367 = t854 * t1365;
    let t2370 = t106 * t737;
    let t2371 = t2370 * t2059;
    let t2374 = t858 * t2078;
    (t2362, t2364, t2367, t2371, t2374)
}
