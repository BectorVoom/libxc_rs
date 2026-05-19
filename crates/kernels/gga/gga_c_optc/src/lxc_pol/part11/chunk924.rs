//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 924/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk924<F: Float>(t17314: F, t275: F, t176: F, t16824: F, t16826: F, t16828: F, t16860: F, t16864: F, t16866: F, t16869: F, t16877: F, t17039: F, t17043: F, t17249: F, t277: F, t364: F, t95: F, t962: F, sigma0: F) -> (F, F) {
    let t17315 = t17314 * t275;
    let t17317 = t176 * t17315 * sigma0;
    let t17320 = F::cast_from(0.25844881434903430496e-2_f64) * t95 * t277 * t17249 * t962 + t17317 * t364 / F::new(2.0) + t17043 + t16824 + t16826 + t16828 + t16860 + t16864 - t16866 - t16869 + t16877 + t17039;
    (t17317, t17320)
}
