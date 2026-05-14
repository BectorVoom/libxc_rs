//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 641/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk641<F: Float>(t27185: F, t28: F, t89: F, t23931: F, t27145: F, t27150: F, t27155: F, t27161: F, t27163: F, t27168: F, t27171: F, t27176: F, t27179: F, t27183: F, t27063: F, t27112: F, t27141: F) -> (F, F, F) {
    let t27186 = t28 * t27185;
    let t27187 = t89 * t27186;
    let t27189 = t27145 / 3.0 - t27150 / 2.0 - t27155 / 2.0 - 3.0 / 8.0 * t27161 - t27163 / 18.0 + t27168 / 6.0 + t27171 / 3.0 - 2.0 / 3.0 * t23931 + 2.0 * t27176 - 2.0 / 3.0 * t27179 + 2.0 * t27183 + 2.0 * t27187;
    let t27191 = t27063 + t27112 + t27141 + t27189;
    (t27186, t27187, t27191)
}
