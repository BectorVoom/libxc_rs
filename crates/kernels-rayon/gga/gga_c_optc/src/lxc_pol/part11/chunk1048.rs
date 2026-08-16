//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1048/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1048(t26264: f64, t373: f64, t26261: f64, t56: f64, t8950: f64, t2848: f64, t136: f64, t8425: f64, t22502: f64, t370: f64, t376: f64, t2933: f64, t2972: f64, t393: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26265 = 0.13388493827160493828e1_f64 * t26264;
    let t26266 = f64::powf(t373, -0.25e1_f64);
    let t26313 = 280.0_f64 / 81.0_f64 * t26261;
    let t26334 = t56 * t8950;
    let t26335 = t2848 * t2848;
    let t26336 = 1.0_f64 / t26335;
    let t26374 = t136 * t8425;
    let t26424 = 1.0_f64 / t376 / t22502 / t370 / 96.0_f64;
    let t26496 = 0.31310740740740740741e1_f64 * t26261;
    let t26497 = 0.13490888888888888889e1_f64 * t26264;
    let t26593 = t393 / t2972 / t2933;
    (t26265, t26266, t26313, t26334, t26336, t26374, t26424, t26496, t26497, t26593)
}
