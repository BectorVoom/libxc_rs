//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 611/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk611(t106: f64, t1411: f64, t2694: f64, t335: f64, t3853: f64, t4983: f64, t4990: f64, t5049: f64, t908: f64, t1415: f64, t1000: f64, t4776: f64) -> (f64, f64, f64) {
    let t5053 = 0.27818116767324025134e1_f64 * t106 * t4983 * t335 - 0.55636233534648050268e1_f64 * t106 * t3853 * t1411 + 0.55636233534648050268e1_f64 * t106 * t2694 * t4990 - 0.27818116767324025134e1_f64 * t106 * t908 * t5049;
    let t5059 = t1415 * t1415;
    let t5064 = t1000 * t4776;
    (t5053, t5059, t5064)
}
