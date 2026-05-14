//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 912/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk912<F: Float>(t38771: F, t57620: F, t73343: F, t73358: F, t73405: F, t86246: F, t86250: F, t86254: F, t86258: F, t86264: F, t86268: F, t86274: F, t86278: F, t86281: F, t86284: F, t46256: F, t46320: F, t57718: F, t59170: F, t73439: F, t73442: F, t74307: F, t74374: F, t74377: F, t86289: F, t86297: F, t86300: F, t86303: F, t86306: F, t86309: F) -> (F, F) {
    let t86386 = 40.0 / 27.0 * t86246 + 8.0 / 3.0 * t86250 - 80.0 / 243.0 * t86254 - t86258 / 9.0 - 8.0 / 3.0 * t73343 + t38771 - 4.0 / 9.0 * t73358 + 8.0 / 3.0 * t86264 + 2.0 / 3.0 * t86268 + 16.0 / 9.0 * t57620 - 8.0 / 27.0 * t73405 + 8.0 * t86274 + 2.0 * t86278 + 3.0 / 4.0 * t86281 - t86284 / 3.0;
    let t86402 = 112.0 / 81.0 * t46256 - t86289 / 3.0 - 8.0 / 27.0 * t57718 + 112.0 / 243.0 * t46320 + 8.0 / 3.0 * t73439 + 4.0 / 9.0 * t73442 - 8.0 / 9.0 * t74307 + 8.0 / 27.0 * t74374 - 16.0 / 9.0 * t86297 - 8.0 / 9.0 * t86300 - 4.0 / 3.0 * t86303 + 4.0 / 9.0 * t86306 - 4.0 / 3.0 * t86309 + 40.0 / 243.0 * t74377 - 8.0 / 9.0 * t59170;
    (t86386, t86402)
}
