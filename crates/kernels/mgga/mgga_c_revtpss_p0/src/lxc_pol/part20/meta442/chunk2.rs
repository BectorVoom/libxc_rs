//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1689/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1689<F: Float>(t12267: F, t81: F, t10321: F, t10326: F, t10336: F, t10380: F, t10381: F, t10389: F, t10392: F, t10398: F, t10401: F, t10407: F, t2251: F, t2258: F, t2260: F, t2263: F, t2291: F, t2292: F, t2299: F, t2306: F, t2312: F, t39443: F, t39449: F, t39457: F, t46001: F, t607: F, t608: F, t628: F, t633: F, t637: F, t641: F, t71: F, t77: F, t85: F) -> F {
    let t46014 = F::cast_from(1.0_f64) / t81 / t12267;
    let t46034 = -t2260 * t2312 / F::cast_from(2.0_f64) - t607 * t10380 * t85 / F::cast_from(3.0_f64) - t10336 * t641 - t2263 * t2312 - t608 * t10407 / F::cast_from(3.0_f64) + t10381 * t641 / F::cast_from(6.0_f64) + t2292 * t2312 / F::cast_from(4.0_f64) + t628 * t10407 / F::cast_from(6.0_f64) + t71 * t77 * (F::cast_from(3640.0_f64) / F::cast_from(81.0_f64) * t46001 * t39443 - F::cast_from(560.0_f64) / F::cast_from(9.0_f64) * t10389 * t2251 * t2258 + F::cast_from(28.0_f64) / F::cast_from(3.0_f64) * t2299 * t39449 + F::cast_from(112.0_f64) / F::cast_from(9.0_f64) * t10392 * t10326 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t633 * t39457 + F::cast_from(3640.0_f64) / F::cast_from(81.0_f64) * t46014 * t39443 + F::cast_from(560.0_f64) / F::cast_from(9.0_f64) * t10398 * t2251 * t2258 + F::cast_from(28.0_f64) / F::cast_from(3.0_f64) * t2306 * t39449 + F::cast_from(112.0_f64) / F::cast_from(9.0_f64) * t10401 * t10326 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t637 * t39457) / F::cast_from(24.0_f64) - t2251 * t2291 * t85 / F::cast_from(2.0_f64) - t10321 * t641;
    t46034
}
