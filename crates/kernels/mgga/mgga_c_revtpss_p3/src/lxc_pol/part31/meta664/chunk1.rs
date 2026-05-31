//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2255/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2255<F: Float>(t28276: F, t4292: F, t572: F, t109291: F, t109293: F, t109295: F, t109299: F, t109305: F, t109307: F, t109310: F, t109315: F, t109319: F, t109322: F, t109327: F, t109330: F, t1918: F, t2040: F, t22559: F, t22565: F, t28246: F, t5802: F, t6948: F, t7324: F, t7944: F) -> F {
    let t109333 = F::cast_from(12.0_f64) * t572 * t28276 * t4292;
    let t109334 = F::cast_from(6.0_f64) * t1918 * t28246 + F::cast_from(12.0_f64) * t2040 * t22559 + F::cast_from(6.0_f64) * t2040 * t22565 + F::cast_from(12.0_f64) * t5802 * t7944 + F::cast_from(3.0_f64) * t6948 * t7324 + t109291 + t109293 + t109295 + t109299 + t109305 + t109307 + t109310 + t109315 + t109319 + t109322 + t109327 + t109330 + t109333;
    t109334
}
