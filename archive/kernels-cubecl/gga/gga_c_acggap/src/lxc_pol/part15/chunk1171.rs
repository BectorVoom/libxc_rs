//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1171/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1171<F: Float>(t7433: F, t9593: F, t5612: F, t7822: F, t5743: F, t8511: F, t2068: F, t8480: F, t8907: F, t8911: F, t13364: F, t31115: F, t40116: F) -> (F, F, F, F, F, F) {
    let t40270 = t7433 * t9593;
    let t40272 = t7822 * t5612;
    let t40274 = t8511 * t5743;
    let t40277 = t2068 * t8480 * t8907;
    let t40280 = t2068 * t8480 * t8911;
    let t40283 = t31115 * t13364 * t40116;
    (t40270, t40272, t40274, t40277, t40280, t40283)
}
