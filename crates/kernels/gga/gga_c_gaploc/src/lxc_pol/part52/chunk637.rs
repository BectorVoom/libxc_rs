//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 637/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk637<F: Float>(t13937: F, t943: F, t12405: F, t12784: F, t13288: F, t13291: F, t13292: F, t13293: F, t13294: F, t13295: F) -> (F, F) {
    let t13938 = t943 * t13937;
    let t14266 = t13288 + 2.0 * t12784 - 2.0 * t12405 - t13291 - t13292 + t13293 + t13294 + t13295;
    (t13938, t14266)
}
