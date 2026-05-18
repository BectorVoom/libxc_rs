//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 121/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk121<F: Float>(t116: F, t288: F, t277: F, t124: F, t234: F, t269: F, t271: F) -> (F, F, F) {
    let t289 = t116 * t288;
    let t293 = F::new(1.0) / t277;
    let t297 = f64::exp(-F::new(0.12897460341341234505e3) * (-t234 + t269 + t271) * t293 * t124);
    (t289, t293, t297)
}
