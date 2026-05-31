//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1379/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1379<F: Float>(t12255: F, t13859: F, t14797: F, t3990: F, t11365: F, t51898: F, t12215: F, t3965: F, t15317: F, t51682: F, t1193: F, t12164: F, t14397: F, t3040: F, t335: F, t338: F, t51978: F, t54639: F, t55947: F, t57657: F, t57661: F, t57663: F, t57666: F, t57668: F, t57671: F, t57674: F) -> F {
    let t57678 = t13859 * t3990 * t14797 * t12255;
    let t57685 = t51898 * t11365;
    let t57687 = t3965 * t12215;
    let t57689 = t51682 * t15317;
    let t57691 = t57657 / F::cast_from(48.0_f64) - t3040 * t14397 / F::cast_from(48.0_f64) + t57661 / F::cast_from(48.0_f64) + t57663 / F::cast_from(96.0_f64) - t57666 / F::cast_from(96.0_f64) - t57668 / F::cast_from(24.0_f64) + t57671 / F::cast_from(48.0_f64) - t51978 - t57674 / F::cast_from(8.0_f64) - t57678 / F::cast_from(384.0_f64) - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t54639 + t55947 - t335 * t338 * t12164 * t1193 / F::cast_from(96.0_f64) - t57685 / F::cast_from(4.0_f64) + t57687 / F::cast_from(24.0_f64) - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t57689;
    t57691
}
