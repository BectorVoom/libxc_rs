//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 421/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk421(t349: f64, t972: f64, t346: f64, t2257: f64, t2280: f64, t92: f64, t93: f64, t136: f64, t3: f64, t287: f64, t529: f64, t362: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t2300 = 1.0_f64 / t972 / t349;
    let t2301 = t346 * t2300;
    let t2305 = 0.96922222222222222222e3_f64 * t2257;
    let t2310 = 0.13111111111111111111e3_f64 * t2280;
    let t2325 = 1.0_f64 / t92 / pi * t93;
    let t2335 = t136 * t3;
    let t2350 = t529 * t287;
    let t2351 = t2350 * t362;
    (t2300, t2301, t2305, t2310, t2325, t2335, t2350, t2351)
}
