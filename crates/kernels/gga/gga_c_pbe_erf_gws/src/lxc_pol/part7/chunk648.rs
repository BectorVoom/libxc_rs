//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 648/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk648<F: Float>(t22: F, t5399: F, t219: F, t5063: F, t4367: F, t639: F, t1774: F, t586: F) -> (F, F, F, F, F, F) {
    let t5400 = t22 * t5399;
    let t5401 = t219 * t5063;
    let t5402 = t5401 * t4367;
    let t5403 = t5400 * t5402;
    let t5405 = 32.0 / 81.0 * t639 * t5403;
    let t5406 = t1774 * t586;
    (t5400, t5401, t5402, t5403, t5405, t5406)
}
