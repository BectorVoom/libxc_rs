//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1042/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1042(t3351: f64, t3352: f64, t511: f64, t6434: f64, t1971: f64, t46846: f64, t7190: f64, t6400: f64, t880: f64, t7720: f64, t9938: f64, t1356: f64, t1364: f64, t1550: f64, t2024: f64, t2604: f64, t289: f64, t35799: f64, t36331: f64, t3928: f64, t46324: f64, t46679: f64, t47119: f64, t47124: f64, t47133: f64, t47135: f64, t47138: f64, t47142: f64, t6403: f64, t6412: f64, t665: f64, t9858: f64) -> f64 {
    let t47146 = t3351 * t3352 * t511 * t6434;
    let t47152 = t3351 * t1971 * t7190 * t46846;
    let t47156 = t3351 * t1971 * t880 * t6400;
    let t47158 = t7720 * t9938;
    let t47160 = -0.23948483403727617128e0_f64 * t1364 * t46679 - 0.85129199786595678796e-5_f64 * t47119 + 0.35922725105591425692e0_f64 * t3928 * t665 * t6403 + 0.23948483403727617128e0_f64 * t1550 * t2024 * t47124 - 0.23948483403727617128e0_f64 * t1550 * t665 * t6412 + t35799 - 0.11974241701863808564e0_f64 * t2604 * t9858 - 0.29795219925308487578e-4_f64 * t47133 - 0.2363e1_f64 * t289 * t47135 + 0.99317399751028291929e-5_f64 * t47138 - 0.76616279807936110914e-4_f64 * t47142 - 0.76616279807936110914e-4_f64 * t47146 + 0.79828278012425390428e-1_f64 * t1356 * t46324 - t36331 - 0.51077519871957407276e-4_f64 * t47152 + 0.10215503974391481455e-3_f64 * t47156 + 0.51077519871957407276e-4_f64 * t47158;
    t47160
}
