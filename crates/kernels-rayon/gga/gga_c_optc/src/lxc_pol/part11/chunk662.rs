//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 662/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk662(t106: f64, t1147: f64, t1550: f64, t3170: f64, t4403: f64, t470: f64, t5344: f64, t5351: f64, t5430: f64, t115: f64, t5274: f64, t5: f64) -> (f64, f64) {
    let t5434 = 0.27818116767324025134e1_f64 * t106 * t5344 * t470 - 0.55636233534648050268e1_f64 * t106 * t4403 * t1550 + 0.55636233534648050268e1_f64 * t106 * t3170 * t5351 - 0.27818116767324025134e1_f64 * t106 * t1147 * t5430;
    let t5439 = t5274 * t115;
    let t5440 = t5439 * t5;
    (t5434, t5440)
}
