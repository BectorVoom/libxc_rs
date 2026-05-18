//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 935/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk935<F: Float>(t2273: F, t8640: F, t70: F, t8119: F, t179: F, t37355: F, t2277: F, t2271: F, t37357: F, t37362: F, t37391: F, t39402: F, t39404: F, t39413: F, t39415: F, t39417: F, t39422: F, t631: F, t632: F, t637: F, t72: F, t8624: F, t8633: F, t8660: F, t8709: F) -> F {
    let t39424 = t8640 * t2273;
    let t39430 = t70 * t8119;
    let t39431 = t179 * t37355;
    let t39436 = t8640 * t2277;
    let t39438 = -F::new(6.0) * t631 * t637 * t8624 * t8709 + F::new(12.0) * t39402 - F::new(8.0) / F::new(3.0) * t39404 - F::new(4.0) * t631 * t72 * t8660 * t37357 - t631 * t72 * t2271 * t37362 + F::new(8.0) / F::new(9.0) * t39413 - F::new(16.0) / F::new(81.0) * t39415 - F::new(8.0) / F::new(9.0) * t631 * t8633 * t39417 * t37357 - F::new(160.0) / F::new(81.0) * t39422 - F::new(20.0) / F::new(9.0) * t39424 + t631 * t72 * t632 * t37391 / F::new(6.0) + F::new(14.0) / F::new(81.0) * t631 * t39430 * t39431 * t37357 + F::new(10.0) / F::new(9.0) * t39436;
    t39438
}
