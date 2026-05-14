//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1201/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1201<F: Float>(t11365: F, t51898: F, t12215: F, t3965: F, t15317: F, t51682: F, t1193: F, t12164: F, t14397: F, t3040: F, t335: F, t338: F, t51978: F, t54639: F, t55947: F, t57657: F, t57661: F, t57663: F, t57666: F, t57668: F, t57671: F, t57674: F, t57678: F) -> (F,) {
    let t57685 = t51898 * t11365;
    let t57687 = t3965 * t12215;
    let t57689 = t51682 * t15317;
    let t57691 = t57657 / 48.0 - t3040 * t14397 / 48.0 + t57661 / 48.0 + t57663 / 96.0 - t57666 / 96.0 - t57668 / 24.0 + t57671 / 48.0 - t51978 - t57674 / 8.0 - t57678 / 384.0 - 35.0 / 216.0 * t54639 + t55947 - t335 * t338 * t12164 * t1193 / 96.0 - t57685 / 4.0 + t57687 / 24.0 - 7.0 / 48.0 * t57689;
    (t57691,)
}
