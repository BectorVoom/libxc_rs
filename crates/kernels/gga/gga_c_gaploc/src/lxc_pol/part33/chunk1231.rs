//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1231/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1231<F: Float>(t31021: F, t31024: F, t31040: F, t31044: F, t31045: F, t31050: F, t31053: F, t31056: F, t34855: F, t34860: F, t34863: F, t34866: F, t34869: F, t34874: F, t34877: F, t38277: F, t4820: F, t6824: F) -> (F,) {
    let t38769 = 0.76685851907841499354e0 * t31021 + t31024 - t34855 + t34860 + t34863 - t34866 - t34869 + t31040 + t31044 - 0.51123901271894332903e1 * t31045 - 0.15889106645266856297e0 * t6824 * t4820 * t38277 - t31050 + t31053 - t31056 - t34874 + t34877;
    (t38769,)
}
