//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1082/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1082<F: Float>(t92185: F, t1882: F, t23033: F, t22963: F, t376: F, t89: F, t1307: F, t7800: F, t5693: F, t8232: F, t23054: F, t23071: F, t23067: F, t23063: F, t22967: F, t23009: F, t23011: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t92186 = 14.0 / 81.0 * t92185;
    let t92191 = t1882 * t23033;
    let t92192 = 2.0 / 9.0 * t92191;
    let t92194 = t89 * t376 * t22963;
    let t92195 = 2.0 * t92194;
    let t92196 = t1307 * t7800;
    let t92201 = t8232 * t5693;
    let t92218 = t23054 * t23071;
    let t92219 = t92218 / 9.0;
    let t92237 = t23054 * t23067;
    let t92238 = t92237 / 27.0;
    let t92239 = t23054 * t23063;
    let t92240 = t92239 / 18.0;
    let t92251 = t1882 * t22967;
    let t92252 = 4.0 / 9.0 * t92251;
    let t92254 = t23009 * t376 * t23011;
    (t92186, t92191, t92192, t92194, t92195, t92196, t92201, t92218, t92219, t92237, t92238, t92239, t92240, t92251, t92252, t92254)
}
