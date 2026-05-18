//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1378/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1378<F: Float>(t11378: F, t53566: F, t14733: F, t9917: F, t9923: F, t2409: F, t36046: F, t3965: F, t12257: F, t3959: F, t36007: F, t53840: F, t53841: F, t9872: F) -> (F, F, F, F, F, F, F) {
    let t57657 = t53566 * t11378;
    let t57661 = t14733 * t9917;
    let t57663 = t14733 * t9923;
    let t57666 = t3965 * t2409 * t36046;
    let t57668 = t3959 * t12257;
    let t57671 = t3965 * t2409 * t36007;
    let t57674 = t53840 * t53841 * t9872;
    (t57657, t57661, t57663, t57666, t57668, t57671, t57674)
}
