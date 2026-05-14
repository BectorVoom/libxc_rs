//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1173/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1173<F: Float>(t2409: F, t36046: F, t3965: F, t12257: F, t3959: F, t36007: F, t53840: F, t53841: F, t9872: F, t12255: F, t13859: F, t14797: F, t3990: F, t11365: F, t51898: F, t12215: F) -> (F, F, F, F, F, F, F) {
    let t57666 = t3965 * t2409 * t36046;
    let t57668 = t3959 * t12257;
    let t57671 = t3965 * t2409 * t36007;
    let t57674 = t53840 * t53841 * t9872;
    let t57678 = t13859 * t3990 * t14797 * t12255;
    let t57685 = t51898 * t11365;
    let t57687 = t3965 * t12215;
    (t57666, t57668, t57671, t57674, t57678, t57685, t57687)
}
