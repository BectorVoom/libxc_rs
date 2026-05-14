//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 647/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk647<F: Float>(t14364: F, t808: F, t568: F, t836: F, t314: F, t313: F, t739: F, t531: F, t2958: F, t3720: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14365 = t808 * t14364;
    let t14366 = t568 * t14365;
    let t14369 = t836 * t14364;
    let t14370 = t568 * t14369;
    let t14373 = t314 * t14364;
    let t14374 = t313 * t14373;
    let t14377 = t739 * t14364;
    let t14378 = t531 * t14377;
    let t14384 = t2958 * t3720;
    (t14365, t14366, t14369, t14370, t14373, t14374, t14377, t14378, t14384)
}
