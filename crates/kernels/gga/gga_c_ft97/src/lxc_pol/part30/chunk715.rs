//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 715/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk715<F: Float>(t29097: F, t29145: F, t29197: F, t29241: F, t29284: F, t29336: F, t29374: F, t29407: F, t18986: F, t2: F, t4: F, t26: F) -> (F, F, F) {
    let t29410 = t29097 + t29145 + t29197 + t29241 + t29284 + t29336 + t29374 + t29407;
    let t29414 = t18986 * t2;
    let t29415 = t29414 * t4;
    let t29416 = t29415 * t26;
    (t29410, t29414, t29416)
}
