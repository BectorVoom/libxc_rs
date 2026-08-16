//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3252/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3252<F: Float>(t10381: F, t10407: F, t13389: F, t13392: F, t13393: F, t13396: F, t13397: F, t1487: F, t1494: F, t2292: F, t4238: F, t53459: F, t53464: F, t54450: F, t60479: F, t627: F, t628: F, t641: F, t70: F, t71: F, t77: F, t85: F) -> F {
    let t60483 = -t54450 * t70 * t85 / F::cast_from(12.0_f64) - t53464 * t70 * t85 / F::cast_from(4.0_f64) - t13392 * t627 * t85 / F::cast_from(4.0_f64) - t13393 * t641 / F::cast_from(4.0_f64) - t53459 * t70 * t85 / F::cast_from(4.0_f64) - t13396 * t627 * t85 / F::cast_from(2.0_f64) - t13397 * t641 / F::cast_from(2.0_f64) + t1487 * t10407 / F::cast_from(24.0_f64) + t10381 * t1494 / F::cast_from(24.0_f64) + t2292 * t4238 / F::cast_from(8.0_f64) + t628 * t13389 / F::cast_from(8.0_f64) + t71 * t77 * t60479 / F::cast_from(24.0_f64);
    t60483
}
