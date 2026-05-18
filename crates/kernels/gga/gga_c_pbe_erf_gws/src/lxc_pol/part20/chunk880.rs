//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 880/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk880<F: Float>(t3242: F, t6627: F, t2319: F, t3295: F, t1105: F, t2264: F, t899: F, t923: F, t3249: F, t6636: F, t6684: F, t2323: F, t3279: F) -> (F, F, F, F, F, F) {
    let t9598 = F::new(7.0) / F::new(288.0) * t6627 * t3242;
    let t9601 = F::new(7.0) / F::new(1152.0) * t2319 * t3295;
    let t9607 = t1105 * param_a_c;
    let t9630 = t899 * t2264 * t923;
    let t9632 = F::new(7.0) / F::new(384.0) * t9630 * t3249;
    let t9637 = t6684 * t6636;
    let t9645 = F::new(35.0) / F::new(576.0) * t2323 * t3279;
    (t9598, t9601, t9607, t9632, t9637, t9645)
}
