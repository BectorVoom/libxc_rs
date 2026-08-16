//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 700/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk700(t1504: f64, t481: f64, t100: f64, t143: f64, t2035: f64, t281: f64, t475: f64, t523: f64, t5636: f64, t5645: f64, t5650: f64, t5653: f64, t5657: f64, t5661: f64, t5666: f64, t5670: f64, t5674: f64, t5678: f64, t5680: f64) -> (f64, f64, f64) {
    let t5683 = t1504 * t481;
    let t5684 = t5683 * t100;
    let t5687 = -0.11974234010254609094e-1_f64 * t281 * t5636 + 3.0_f64 * t475 * t143 * t5645 - 9.0_f64 * t5650 * t5653 + 9.0_f64 * t2035 * t5657 - 2.0_f64 * t523 * t5661 - 0.16213771438917426213e0_f64 * t5666 + 0.40679438125041687114e-2_f64 * t5670 + 0.59450495276030562782e0_f64 * t5674 - 0.87170224553660758101e-3_f64 * t5678 + 9.0_f64 * t2035 * t5680 + 6.0_f64 * t5684 * t143;
    (t5683, t5684, t5687)
}
