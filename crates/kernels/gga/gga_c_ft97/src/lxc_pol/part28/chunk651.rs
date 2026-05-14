//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 651/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk651<F: Float>(t23931: F, t27145: F, t27150: F, t27155: F, t27161: F, t27163: F, t27168: F, t27171: F, t27176: F, t27179: F, t27183: F, t27187: F, t27351: F, t27364: F, t27376: F, t143: F, t160: F) -> (F, F) {
    let t27389 = t27145 / 9.0 - t27150 / 6.0 - t27155 / 6.0 - t27161 / 8.0 - t27163 / 54.0 + t27168 / 18.0 + t27171 / 9.0 - 2.0 / 9.0 * t23931 + 2.0 / 3.0 * t27176 - 2.0 / 9.0 * t27179 + 2.0 / 3.0 * t27183 + 2.0 / 3.0 * t27187;
    let t27391 = t27351 + t27364 + t27376 + t27389;
    let t27393 = t143 * t27391 * t160;
    (t27391, t27393)
}
