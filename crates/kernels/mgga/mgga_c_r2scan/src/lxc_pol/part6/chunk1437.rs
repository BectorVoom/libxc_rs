//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1437/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1437<F: Float>(t7494: F, t8018: F, t2582: F, t2583: F, t6848: F, t2139: F, t2294: F, t8106: F, t19986: F, t2122: F, t2124: F, t22788: F, t22793: F, t22797: F, t22800: F, t22803: F, t24609: F, t24624: F, t2550: F, t2572: F, t2584: F, t2598: F, t360: F, t571: F, t6359: F, t6364: F, t6370: F, t7533: F, t910: F) -> (F,) {
    let t27015 = t7494 * t8018;
    let t27022 = t2582 * t6848 * t2583;
    let t27023 = 0.12713391885412927226e1 * t27022;
    let t27025 = t2139 * t2294 * t8106;
    let t27052 = -0.38415120233790484326e0 * t27015 + 0.54878743191129263322e-1 * t2122 * t2124 * t2550 * t24609 - t27023 - 0.10401866088065122276e1 * t27025 + 0.60677552180379879941e0 * t2598 * t360 * t2572 * t6370 + 0.32927245914677557992e0 * t2122 * t2124 * t6359 * t910 * t6364 - 0.32927245914677557992e0 * t2122 * t2124 * t7533 * t6370 - 0.13002332610081402845e0 * t571 * t19986 * t2584 + 0.86682217400542685632e-1 * t2598 * t360 * t2572 * t24624 - 0.58218257753910989057e-2 * t22788 - 0.1047928639570397803e0 * t22793 + 0.34930954652346593433e-1 * t22797 - 0.22084125774650235182e1 * t22800 + 0.32927245914677557992e-1 * t22803;
    (t27052,)
}
