//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 738/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk738<F: Float>(t136: F, t162: F, t6165: F, t159: F, t148: F, t151: F, t6568: F, t2168: F, t6778: F, t686: F, t6919: F, t6933: F, t6938: F, t705: F, t7074: F, t7076: F, t7078: F, t7083: F, t7086: F) -> F {
    let t7089 = t6165 * t136 * t162;
    let t7091 = F::cast_from(0.13322897401211865505e1_f64) * t159 * t7089;
    let t7094 = F::cast_from(0.29299173910028776472e1_f64) * t148 * t6568 * t151;
    let t7101 = -F::cast_from(0.40568086952347536654e1_f64) * t7074 + F::cast_from(0.12170426085704260996e1_f64) * t7076 - F::cast_from(0.2115989587251296286e1_f64) * t7078 - F::cast_from(0.90685268025055555116e0_f64) * t705 * t6919 - F::cast_from(0.20863587575493018851e1_f64) * t686 * t7083 - F::cast_from(0.36511278257112782988e1_f64) * t7086 - t7091 - t7094 - F::cast_from(0.90685268025055555117e0_f64) * t2168 * t6933 + F::cast_from(0.18137053605011111023e0_f64) * t2168 * t6938 - F::cast_from(0.45342634012527777558e-1_f64) * t2168 * t6778;
    t7101
}
