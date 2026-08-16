//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1042/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1042<F: Float>(t3351: F, t3352: F, t511: F, t6434: F, t1971: F, t46846: F, t7190: F, t6400: F, t880: F, t7720: F, t9938: F, t1356: F, t1364: F, t1550: F, t2024: F, t2604: F, t289: F, t35799: F, t36331: F, t3928: F, t46324: F, t46679: F, t47119: F, t47124: F, t47133: F, t47135: F, t47138: F, t47142: F, t6403: F, t6412: F, t665: F, t9858: F) -> F {
    let t47146 = t3351 * t3352 * t511 * t6434;
    let t47152 = t3351 * t1971 * t7190 * t46846;
    let t47156 = t3351 * t1971 * t880 * t6400;
    let t47158 = t7720 * t9938;
    let t47160 = -F::cast_from(0.23948483403727617128e0_f64) * t1364 * t46679 - F::cast_from(0.85129199786595678796e-5_f64) * t47119 + F::cast_from(0.35922725105591425692e0_f64) * t3928 * t665 * t6403 + F::cast_from(0.23948483403727617128e0_f64) * t1550 * t2024 * t47124 - F::cast_from(0.23948483403727617128e0_f64) * t1550 * t665 * t6412 + t35799 - F::cast_from(0.11974241701863808564e0_f64) * t2604 * t9858 - F::cast_from(0.29795219925308487578e-4_f64) * t47133 - F::cast_from(0.2363e1_f64) * t289 * t47135 + F::cast_from(0.99317399751028291929e-5_f64) * t47138 - F::cast_from(0.76616279807936110914e-4_f64) * t47142 - F::cast_from(0.76616279807936110914e-4_f64) * t47146 + F::cast_from(0.79828278012425390428e-1_f64) * t1356 * t46324 - t36331 - F::cast_from(0.51077519871957407276e-4_f64) * t47152 + F::cast_from(0.10215503974391481455e-3_f64) * t47156 + F::cast_from(0.51077519871957407276e-4_f64) * t47158;
    t47160
}
