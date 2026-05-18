//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 792/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk792<F: Float>(t12527: F, t21370: F, t2464: F, t2465: F, t6914: F, t9176: F, t20884: F, t30845: F, t900: F, t9086: F, t9561: F, t123: F, t883: F, t9127: F) -> (F, F, F, F, F, F) {
    let t40178 = t21370 * t12527;
    let t40182 = t6914 * t2464 * t2465 * t9176;
    let t40184 = t30845 * t20884;
    let t40186 = t900 * t9086;
    let t40187 = t9561 * t40186;
    let t40190 = t9127 * t123 * t883;
    (t40178, t40182, t40184, t40186, t40187, t40190)
}
