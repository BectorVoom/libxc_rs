//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3248/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3248<F: Float>(t10321: F, t13335: F, t13340: F, t13400: F, t13405: F, t1494: F, t2251: F, t2252: F, t2291: F, t2312: F, t36: F, t38: F, t4181: F, t4217: F, t4218: F, t4238: F, t49889: F, t60297: F, t60330: F, t627: F, t641: F, t70: F, t85: F) -> F {
    let t60360 = t38 * (t60297 + t60330) * t85 / F::new(24.0) + t13335 * t641 / F::new(8.0) - t2251 * t4217 * t85 / F::new(4.0) - t13340 * t641 / F::new(4.0) - t10321 * t1494 / F::new(4.0) - t2252 * t4238 / F::new(4.0) + t4218 * t2312 / F::new(8.0) - t4181 * t2291 * t85 / F::new(4.0) - t13400 * t641 / F::new(2.0) - t36 * t49889 * t70 * t85 / F::new(12.0) - t13405 * t627 * t85 / F::new(4.0);
    t60360
}
