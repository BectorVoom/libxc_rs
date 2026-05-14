//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 958/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk958<F: Float>(t25423: F, t8126: F, t19: F, t769: F, t3906: F, t2662: F, t322: F, t8192: F) -> (F, F, F, F, F) {
    let t25424 = t8126 * t25423;
    let t25427 = t19 * t769;
    let t25453 = t3906 * t25423;
    let t25458 = t2662 * t25423;
    let t25560 = t8192 * t322;
    (t25424, t25427, t25453, t25458, t25560)
}
