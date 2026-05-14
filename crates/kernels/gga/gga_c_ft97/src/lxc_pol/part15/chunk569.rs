//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 569/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk569<F: Float>(t10: F, t296: F, t3050: F, t2404: F, t798: F, t295: F, t9577: F, t2344: F) -> (F, F, F, F, F) {
    let t10397 = t10 * t3050 * t296;
    let t10398 = 14.0 / 81.0 * t10397;
    let t10409 = t2404 * t798;
    let t10414 = t295 * t9577;
    let t10478 = t2344 * t798;
    (t10397, t10398, t10409, t10414, t10478)
}
