//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1135/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1135<F: Float>(t13396: F, t70: F, t4181: F, t627: F, t13312: F, t36: F, t4187: F, t1470: F, t2291: F, t13389: F, t13393: F, t1494: F, t2292: F, t4182: F, t4188: F, t4191: F, t4238: F, t628: F, t641: F, t71: F, t85: F) -> F {
    let t13397 = t13396 * t70;
    let t13400 = t4181 * t627;
    let t13405 = t36 * t13312;
    let t13406 = t13405 * t70;
    let t13409 = t4187 * t627;
    let t13414 = t1470 * t2291;
    let t13419 = t2292 * t1494 / F::new(24.0) + t628 * t4238 / F::new(12.0) + t71 * t13389 / F::new(24.0) - t13393 * t85 / F::new(12.0) - t13397 * t85 / F::new(6.0) - t13400 * t85 / F::new(6.0) - t4182 * t641 / F::new(6.0) - t13406 * t85 / F::new(12.0) - t13409 * t85 / F::new(6.0) - t4188 * t641 / F::new(6.0) - t13414 * t85 / F::new(12.0) - t4191 * t641 / F::new(6.0);
    t13419
}
