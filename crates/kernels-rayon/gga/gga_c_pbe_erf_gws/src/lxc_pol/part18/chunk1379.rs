//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1379/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1379(t12255: f64, t13859: f64, t14797: f64, t3990: f64, t11365: f64, t51898: f64, t12215: f64, t3965: f64, t15317: f64, t51682: f64, t1193: f64, t12164: f64, t14397: f64, t3040: f64, t335: f64, t338: f64, t51978: f64, t54639: f64, t55947: f64, t57657: f64, t57661: f64, t57663: f64, t57666: f64, t57668: f64, t57671: f64, t57674: f64) -> f64 {
    let t57678 = t13859 * t3990 * t14797 * t12255;
    let t57685 = t51898 * t11365;
    let t57687 = t3965 * t12215;
    let t57689 = t51682 * t15317;
    let t57691 = t57657 / 48.0_f64 - t3040 * t14397 / 48.0_f64 + t57661 / 48.0_f64 + t57663 / 96.0_f64 - t57666 / 96.0_f64 - t57668 / 24.0_f64 + t57671 / 48.0_f64 - t51978 - t57674 / 8.0_f64 - t57678 / 384.0_f64 - 35.0_f64 / 216.0_f64 * t54639 + t55947 - t335 * t338 * t12164 * t1193 / 96.0_f64 - t57685 / 4.0_f64 + t57687 / 24.0_f64 - 7.0_f64 / 48.0_f64 * t57689;
    t57691
}
