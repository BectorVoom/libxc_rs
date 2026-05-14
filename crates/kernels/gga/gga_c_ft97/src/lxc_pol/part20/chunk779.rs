//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 779/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk779<F: Float>(t24537: F, t24500: F, t24505: F, t24510: F, t24514: F, t24517: F, t24522: F, t24524: F, t24529: F, t24534: F, t24541: F, t24544: F, t24549: F, t24553: F, t24557: F, t24561: F) -> (F, F) {
    let t24642 = 2.0 / 27.0 * t24537;
    let t24649 = -4.0 / 9.0 * t24500 + t24505 / 12.0 - t24510 - t24514 / 6.0 + t24517 / 9.0 + 2.0 / 27.0 * t24522 - 2.0 / 27.0 * t24524 + 2.0 / 9.0 * t24529 - 2.0 / 9.0 * t24534 - t24642 + t24541 / 9.0 - t24544 / 27.0 + t24549 / 9.0 + t24553 / 18.0 - 2.0 * t24557 + t24561 / 27.0;
    (t24642, t24649)
}
