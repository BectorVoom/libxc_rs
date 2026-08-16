//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 646/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk646<F: Float>(t3237: F, t3245: F, t1115: F, t2367: F, t1162: F, t3097: F, t914: F, t3088: F, t1172: F, t2586: F, t1170: F, t1152: F) -> (F, F, F, F, F, F) {
    let t3246 = t3245 * t3237;
    let t3249 = t2367 * t1115;
    let t3250 = t1162 * t3249;
    let t3252 = t914 * t3097;
    let t3255 = t914 * t3088;
    let t3258 = t2586 * t1172;
    let t3259 = t1170 * t3258;
    let t3261 = t2367 * t1152;
    (t3246, t3250, t3252, t3255, t3259, t3261)
}
