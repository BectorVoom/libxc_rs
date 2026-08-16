//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 738/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk738(t136: f64, t162: f64, t6165: f64, t159: f64, t148: f64, t151: f64, t6568: f64, t2168: f64, t6778: f64, t686: f64, t6919: f64, t6933: f64, t6938: f64, t705: f64, t7074: f64, t7076: f64, t7078: f64, t7083: f64, t7086: f64) -> f64 {
    let t7089 = t6165 * t136 * t162;
    let t7091 = 0.13322897401211865505e1_f64 * t159 * t7089;
    let t7094 = 0.29299173910028776472e1_f64 * t148 * t6568 * t151;
    let t7101 = -0.40568086952347536654e1_f64 * t7074 + 0.12170426085704260996e1_f64 * t7076 - 0.2115989587251296286e1_f64 * t7078 - 0.90685268025055555116e0_f64 * t705 * t6919 - 0.20863587575493018851e1_f64 * t686 * t7083 - 0.36511278257112782988e1_f64 * t7086 - t7091 - t7094 - 0.90685268025055555117e0_f64 * t2168 * t6933 + 0.18137053605011111023e0_f64 * t2168 * t6938 - 0.45342634012527777558e-1_f64 * t2168 * t6778;
    t7101
}
