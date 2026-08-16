//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 403/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk403<F: Float>(t2268: F, t3347: F, t2778: F, t883: F, t2325: F, t882: F, t2787: F, t874: F) -> (F, F, F, F) {
    let t3349 = F::cast_from(0.85365019907028448797e-1_f64) * t2268 * t3347;
    let t3350 = t883 * t2778;
    let t3351 = t2325 * t3350;
    let t3352 = t882 * t3351;
    let t3353 = F::cast_from(0.11856252764865062333e-2_f64) * t3352;
    let t3354 = t2787 * t874;
    (t3349, t3351, t3353, t3354)
}
