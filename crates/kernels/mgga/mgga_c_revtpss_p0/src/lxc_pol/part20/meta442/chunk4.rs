//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1691/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1691<F: Float>(t10317: F, t10318: F, t10326: F, t10327: F, t10328: F, t10331: F, t1927: F, t2252: F, t2258: F, t2259: F, t2291: F, t2312: F, t36: F, t38: F, t39449: F, t39457: F, t46091: F, t606: F, t627: F, t641: F, t6977: F, t70: F, t72: F, t85: F) -> F {
    let t46119 = -t2252 * t2312 / F::new(2.0) + t38 * t46091 * t85 / F::new(24.0) - t39449 * t70 * t85 / F::new(4.0) - t606 * t627 * t72 * t10318 - t10317 * t6977 * t2258 - t10317 * t1927 * t10326 / F::new(3.0) - t36 * t39457 * t70 * t85 / F::new(12.0) - t10327 * t627 * t85 / F::new(3.0) - t10328 * t641 / F::new(3.0) - t2259 * t2291 * t85 / F::new(2.0) - t10331 * t641;
    t46119
}
