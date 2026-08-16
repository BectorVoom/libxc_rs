//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 648/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk648(t106: f64, t1147: f64, t1182: f64, t3160: f64, t3164: f64, t3170: f64, t3171: f64, t3264: f64, t470: f64, t1207: f64, t176: f64, t1219: f64) -> (f64, f64, f64) {
    let t3268 = 0.27818116767324025134e1_f64 * t106 * t3160 * t470 - 0.55636233534648050268e1_f64 * t106 * t3164 * t1182 + 0.55636233534648050268e1_f64 * t106 * t3170 * t3171 - 0.27818116767324025134e1_f64 * t106 * t1147 * t3264;
    let t3273 = t176 * t1207;
    let t3274 = t3273 * t1219;
    (t3268, t3273, t3274)
}
