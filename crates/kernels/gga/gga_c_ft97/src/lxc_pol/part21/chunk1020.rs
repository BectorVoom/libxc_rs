//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1020/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1020<F: Float>(t1317: F, t1318: F, t7943: F, t1307: F, t7800: F, t5693: F, t8232: F, t1608: F, t1689: F, t5584: F, t1609: F, t22563: F, t7837: F, t5532: F, t77: F, t1669: F, t22760: F) -> (F, F, F, F, F, F, F, F, F) {
    let t92185 = t1317 * t7943 * t1318;
    let t92186 = 14.0 / 81.0 * t92185;
    let t92196 = t1307 * t7800;
    let t92201 = t8232 * t5693;
    let t92202 = 4.0 / 27.0 * t92201;
    let t92278 = t1608 * t5584 * t1689;
    let t92303 = t7837 * t22563 * t1609;
    let t92339 = t77 * t5532;
    let t92348 = t1669 * t22760;
    (t92185, t92186, t92196, t92201, t92202, t92278, t92303, t92339, t92348)
}
