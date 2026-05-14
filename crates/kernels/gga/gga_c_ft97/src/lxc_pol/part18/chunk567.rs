//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 567/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk567<F: Float>(t184: F, t6731: F, t4: F, t6: F, t1693: F, t43: F, t1711: F, t39: F, t64: F, t173: F, t174: F, t368: F) -> (F, F, F, F, F, F, F) {
    let t6732 = t6731 * t184;
    let t7149 = t4 * t6;
    let t7187 = t1693 * t43;
    let t7201 = t1711 * t39;
    let t7202 = t64 * t7201;
    let t7239 = t173 * t174;
    let t7240 = t368 * t368;
    let t7241 = 1.0 / t7240;
    (t6732, t7149, t7187, t7202, t7239, t7240, t7241)
}
