//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1157/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1157(t1820: f64, t1885: f64, t40566: f64, t995: f64, t42175: f64, t33298: f64, t25514: f64, t34565: f64, t48373: f64, t48377: f64, t48380: f64, t48381: f64, t48382: f64, t48387: f64) -> (f64, f64, f64, f64) {
    let t48392 = 16.0_f64 / 15.0_f64 * t1820 * t1885 * t40566 * t995;
    let t48393 = 64.0_f64 / 45.0_f64 * t42175;
    let t48394 = 32.0_f64 / 135.0_f64 * t33298;
    let t48395 = t48373 + t48377 + t48380 - t48381 + t48382 + 4.0_f64 / 45.0_f64 * t34565 + t48387 - 0.26596355555555555555e0_f64 * t25514 - t48392 + t48393 - t48394;
    (t48392, t48393, t48394, t48395)
}
