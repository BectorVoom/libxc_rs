//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 408/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk408<F: Float>(t2343: F, t3354: F, t2268: F, t3338: F, t493: F) -> (F, F, F) {
    let t3355 = t2343 * t3354;
    let t3357 = F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t3355;
    let t3358 = t493 * t3338;
    (t3355, t3357, t3358)
}
