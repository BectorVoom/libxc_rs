//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 909/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk909<F: Float>(t27: F, t370: F, t85682: F, t89: F, t446: F, t7793: F, t86104: F, t38268: F, t86098: F, t1564: F, t86108: F, t86054: F, t7824: F, t86086: F, t46256: F, t46320: F, t57718: F, t59170: F, t73439: F, t73442: F, t74307: F, t74374: F, t74377: F) -> (F, F, F, F, F, F, F) {
    let t86289 = t89 * t27 * t370 * t85682;
    let t86297 = t446 * t7793 * t86104;
    let t86300 = t446 * t38268 * t86098;
    let t86303 = t446 * t1564 * t86108;
    let t86306 = t446 * t1564 * t86054;
    let t86309 = t446 * t7824 * t86086;
    let t86313 = 112.0 / 27.0 * t46256 - t86289 - 8.0 / 9.0 * t57718 + 112.0 / 81.0 * t46320 + 8.0 * t73439 + 4.0 / 3.0 * t73442 - 8.0 / 3.0 * t74307 + 8.0 / 9.0 * t74374 - 16.0 / 3.0 * t86297 - 8.0 / 3.0 * t86300 - 4.0 * t86303 + 4.0 / 3.0 * t86306 - 4.0 * t86309 + 40.0 / 81.0 * t74377 - 8.0 / 3.0 * t59170;
    (t86289, t86297, t86300, t86303, t86306, t86309, t86313)
}
