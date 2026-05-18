//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 856/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk856<F: Float>(t1413: F, t7282: F, t5089: F, t11: F, t2715: F, t401: F, t2712: F, t1714: F, t7097: F, t5061: F, t7212: F, t657: F, t7264: F) -> (F, F, F, F, F, F, F, F) {
    let t7283 = t7282 * t1413;
    let t7284 = t5089 * t7283;
    let t7285 = t11 * t7284;
    let t7288 = F::new(0.17777777777777777778e-1) * t401 * t2715;
    let t7290 = F::new(0.2962962962962962963e-2) * t401 * t2712;
    let t7291 = t1714 * t7097;
    let t7294 = t5061 * t7283;
    let t7297 = t1714 * t7212;
    let t7300 = t657 * t7264;
    (t7283, t7285, t7288, t7290, t7291, t7294, t7297, t7300)
}
