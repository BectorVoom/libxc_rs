//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2256/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2256<F: Float>(t1913: F, t7956: F, t101563: F, t105814: F, t109278: F, t109289: F, t109334: F, t1458: F, t1464: F, t1914: F, t1921: F, t2038: F, t2045: F, t22533: F, t22571: F, t28235: F, t28283: F, t3: F, t30161: F, t575: F, t5790: F, t5808: F, t6951: F, t7319: F, t7940: F) -> F {
    let t109339 = t1913 * t7956;
    let t109344 = t30161 * t1464 + t22533 * t2045 + t7319 * t6951 + F::cast_from(2.0_f64) * t28235 * t1921 + F::cast_from(2.0_f64) * t5790 * t7956 + t105814 + F::cast_from(2.0_f64) * t1914 * t28283 + t101563 + t1458 * (t109289 + t109334) + t3 * t109278 * t575 + F::cast_from(2.0_f64) * t109339 + F::cast_from(2.0_f64) * t7940 * t5808 + t2038 * t22571;
    t109344
}
