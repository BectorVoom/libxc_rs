//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 427/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk427(t2232: f64, t836: f64, t772: f64, t1: f64, t769: f64, t791: f64, t468: f64, t892: f64, t924: f64, t474: f64, t818: f64, t801: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2233 = t836 * t2232;
    let t2234 = t772 * t2233;
    let t2237 = t769 * t1;
    let t2238 = t791 * t2237;
    let t2239 = t468 * t892;
    let t2242 = t468 * t924;
    let t2245 = t474 * t818;
    let t2246 = t2245 * t801;
    (t2233, t2234, t2238, t2239, t2242, t2245, t2246)
}
