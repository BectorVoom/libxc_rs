//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1034/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1034<F: Float>(t446: F, t7824: F, t86086: F, t46256: F, t46320: F, t57718: F, t59170: F, t73439: F, t73442: F, t74307: F, t74374: F, t74377: F, t86289: F, t86297: F, t86300: F, t86303: F, t86306: F) -> (F, F) {
    let t86309 = t446 * t7824 * t86086;
    let t86313 = F::new(112.0) / F::new(27.0) * t46256 - t86289 - F::new(8.0) / F::new(9.0) * t57718 + F::new(112.0) / F::new(81.0) * t46320 + F::new(8.0) * t73439 + F::new(4.0) / F::new(3.0) * t73442 - F::new(8.0) / F::new(3.0) * t74307 + F::new(8.0) / F::new(9.0) * t74374 - F::new(16.0) / F::new(3.0) * t86297 - F::new(8.0) / F::new(3.0) * t86300 - F::new(4.0) * t86303 + F::new(4.0) / F::new(3.0) * t86306 - F::new(4.0) * t86309 + F::new(40.0) / F::new(81.0) * t74377 - F::new(8.0) / F::new(3.0) * t59170;
    (t86309, t86313)
}
