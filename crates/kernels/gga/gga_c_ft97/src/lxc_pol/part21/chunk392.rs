//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 392/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk392<F: Float>(t1773: F, t3125: F, t3144: F, t4512: F, t4515: F, t4519: F, t4523: F, t4527: F, t4531: F, t462: F, t92: F) -> (F,) {
    let t4533 = t1773 + 2.0 / 9.0 * t3125 + 2.0 / 3.0 * t3144 - 2.0 / 9.0 * t462 * t4512 + 2.0 / 3.0 * t462 * t4515 + 2.0 / 3.0 * t462 * t4519 - t462 * t4523 / 3.0 + 2.0 * t92 * t4527 - t92 * t4531;
    (t4533,)
}
