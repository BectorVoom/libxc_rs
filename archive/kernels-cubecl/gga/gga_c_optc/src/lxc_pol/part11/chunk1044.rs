//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1044/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1044<F: Float>(t2663: F, t276: F, t308: F, t115: F, t282: F, t8206: F, t2769: F, t1659: F, t2746: F, t301: F, t327: F, t24565: F) -> (F, F, F, F, F, F) {
    let t25834 = F::cast_from(1.0_f64) / t2663 / t308 / t276;
    let t25836 = t282 * t25834 * t115;
    let t25837 = t8206 * t25836;
    let t25877 = t2769 * t25836;
    let t25883 = t1659 * t25836;
    let t25939 = F::cast_from(1.0_f64) / t2746 / t327 * t301;
    let t25940 = t25939 * t24565;
    (t25834, t25836, t25837, t25877, t25883, t25940)
}
