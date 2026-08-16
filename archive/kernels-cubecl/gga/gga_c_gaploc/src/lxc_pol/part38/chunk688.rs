//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 688/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk688<F: Float>(t12383: F, t12386: F, t12397: F, t12400: F, t12412: F, t12784: F, t12785: F, t13288: F, t13289: F) -> F {
    let t13291 = F::cast_from(9.0_f64) / F::cast_from(128.0_f64) * t12383;
    let t13292 = F::cast_from(9.0_f64) / F::cast_from(4096.0_f64) * t12386;
    let t13293 = F::cast_from(3.0_f64) / F::cast_from(4096.0_f64) * t12397;
    let t13294 = F::cast_from(3.0_f64) / F::cast_from(128.0_f64) * t12400;
    let t13295 = F::cast_from(4.0_f64) * t12412;
    let t13296 = t13288 + t13289 / F::cast_from(2.0_f64) + t12784 - t12785 - t13291 - t13292 + t13293 + t13294 + t13295;
    t13296
}
