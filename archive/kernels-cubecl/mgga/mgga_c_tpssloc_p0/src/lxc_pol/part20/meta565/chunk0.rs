//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2124/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2124<F: Float>(t10401: F, t10935: F, t3186: F, t3200: F, t11051: F, t3069: F, t10454: F, t3048: F, t10459: F, t3036: F, t3087: F, t3033: F, t3128: F) -> (F, F, F, F, F, F, F) {
    let t42504 = t10935 * t10401;
    let t42505 = t3186 * t42504;
    let t42508 = t3200 * t42504;
    let t42511 = t11051 * t3069;
    let t42514 = t3048 * t10454;
    let t42518 = t3048 * t10459;
    let t42520 = t3087 * t3036;
    let t42522 = t3033 * t3128 * t42520;
    (t42505, t42508, t42511, t42514, t42518, t42520, t42522)
}
