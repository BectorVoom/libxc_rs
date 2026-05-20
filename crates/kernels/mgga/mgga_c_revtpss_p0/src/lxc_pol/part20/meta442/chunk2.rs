//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1689/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1689<F: Float>(t12267: F, t81: F, t10321: F, t10326: F, t10336: F, t10380: F, t10381: F, t10389: F, t10392: F, t10398: F, t10401: F, t10407: F, t2251: F, t2258: F, t2260: F, t2263: F, t2291: F, t2292: F, t2299: F, t2306: F, t2312: F, t39443: F, t39449: F, t39457: F, t46001: F, t607: F, t608: F, t628: F, t633: F, t637: F, t641: F, t71: F, t77: F, t85: F) -> F {
    let t46014 = F::new(1.0) / t81 / t12267;
    let t46034 = -t2260 * t2312 / F::new(2.0) - t607 * t10380 * t85 / F::new(3.0) - t10336 * t641 - t2263 * t2312 - t608 * t10407 / F::new(3.0) + t10381 * t641 / F::new(6.0) + t2292 * t2312 / F::new(4.0) + t628 * t10407 / F::new(6.0) + t71 * t77 * (F::new(3640.0) / F::new(81.0) * t46001 * t39443 - F::new(560.0) / F::new(9.0) * t10389 * t2251 * t2258 + F::new(28.0) / F::new(3.0) * t2299 * t39449 + F::new(112.0) / F::new(9.0) * t10392 * t10326 - F::new(4.0) / F::new(3.0) * t633 * t39457 + F::new(3640.0) / F::new(81.0) * t46014 * t39443 + F::new(560.0) / F::new(9.0) * t10398 * t2251 * t2258 + F::new(28.0) / F::new(3.0) * t2306 * t39449 + F::new(112.0) / F::new(9.0) * t10401 * t10326 + F::new(4.0) / F::new(3.0) * t637 * t39457) / F::new(24.0) - t2251 * t2291 * t85 / F::new(2.0) - t10321 * t641;
    t46034
}
