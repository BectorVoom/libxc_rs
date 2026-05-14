//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 915/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk915<F: Float>(t37293: F, t57627: F, t59392: F, t73343: F, t73358: F, t73405: F, t86246: F, t86250: F, t86254: F, t86258: F, t86264: F, t86268: F, t86274: F, t86278: F, t57767: F, t59426: F, t59486: F, t73439: F, t73442: F, t74307: F, t74374: F, t74377: F, t86289: F, t86297: F, t86300: F, t86303: F, t86306: F, t86309: F) -> (F, F) {
    let t86465 = 20.0 / 27.0 * t86246 + 4.0 / 3.0 * t86250 - 40.0 / 243.0 * t86254 - t86258 / 18.0 - 4.0 / 3.0 * t73343 + t37293 - 2.0 / 9.0 * t73358 + 4.0 / 3.0 * t86264 + t86268 / 3.0 + t59392 - 4.0 / 27.0 * t73405 + 4.0 * t86274 + t86278 + t57627;
    let t86477 = -t86289 / 6.0 - t59426 + t57767 + 4.0 / 3.0 * t73439 + 2.0 / 9.0 * t73442 - 4.0 / 9.0 * t74307 + 4.0 / 27.0 * t74374 - 8.0 / 9.0 * t86297 - 4.0 / 9.0 * t86300 - 2.0 / 3.0 * t86303 + 2.0 / 9.0 * t86306 - 2.0 / 3.0 * t86309 + 20.0 / 243.0 * t74377 - t59486;
    (t86465, t86477)
}
