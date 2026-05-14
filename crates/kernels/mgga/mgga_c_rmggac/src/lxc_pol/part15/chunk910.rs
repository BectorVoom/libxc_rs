//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 910/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk910<F: Float>(t1971: F, t3351: F, t6400: F, t880: F, t7720: F, t9938: F, t1356: F, t1364: F, t1550: F, t2024: F, t2604: F, t289: F, t35799: F, t36331: F, t3928: F, t46324: F, t46679: F, t47119: F, t47124: F, t47133: F, t47135: F, t47138: F, t47142: F, t47146: F, t47152: F, t6403: F, t6412: F, t665: F, t9858: F) -> (F,) {
    let t47156 = t3351 * t1971 * t880 * t6400;
    let t47158 = t7720 * t9938;
    let t47160 = -0.23948483403727617128e0 * t1364 * t46679 - 0.85129199786595678796e-5 * t47119 + 0.35922725105591425692e0 * t3928 * t665 * t6403 + 0.23948483403727617128e0 * t1550 * t2024 * t47124 - 0.23948483403727617128e0 * t1550 * t665 * t6412 + t35799 - 0.11974241701863808564e0 * t2604 * t9858 - 0.29795219925308487578e-4 * t47133 - 0.2363e1 * t289 * t47135 + 0.99317399751028291929e-5 * t47138 - 0.76616279807936110914e-4 * t47142 - 0.76616279807936110914e-4 * t47146 + 0.79828278012425390428e-1 * t1356 * t46324 - t36331 - 0.51077519871957407276e-4 * t47152 + 0.10215503974391481455e-3 * t47156 + 0.51077519871957407276e-4 * t47158;
    (t47160,)
}
