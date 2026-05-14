//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 650/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk650<F: Float>(t25298: F, t25312: F, t25315: F, t25317: F, t25366: F, t29378: F, t29383: F, t29385: F, t29387: F, t29389: F, t29392: F, t29396: F, t29401: F, t29405: F, t446: F, t29097: F, t29145: F, t29197: F, t29241: F, t29284: F, t29336: F, t29374: F) -> (F,) {
    let t29407 = t25298 / 9.0 + t25312 / 27.0 - t446 * t29378 / 3.0 - t25315 / 9.0 + t25317 / 9.0 - t29383 / 9.0 + t29385 / 9.0 + t29387 / 9.0 - t446 * t29389 / 3.0 + t29392 / 9.0 + t25366 / 9.0 + t446 * t29396 / 3.0 + t446 * t29401 / 3.0 - t29405 / 9.0;
    let t29410 = t29097 + t29145 + t29197 + t29241 + t29284 + t29336 + t29374 + t29407;
    (t29410,)
}
