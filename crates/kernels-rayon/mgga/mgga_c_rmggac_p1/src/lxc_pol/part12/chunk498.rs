//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 498/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk498(t1415: f64, t381: f64, t4352: f64, t183: f64, t5415: f64, t155: f64, t1042: f64, t1372: f64, t1138: f64, t1435: f64, t1392: f64, t446: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5425 = t381 * t1415;
    let t5426 = 8.0_f64 * t5425;
    let t5427 = 0.4883052614935078681e-3_f64 * t4352;
    let t5428 = t5415 * t183;
    let t5429 = t155 * t5428;
    let t5432 = t1372 * t1042;
    let t5433 = 0.17315859105681463759e2_f64 * t5432;
    let t5434 = t1435 * t1138;
    let t5435 = 0.24415263074675393405e-3_f64 * t5434;
    let t5436 = t1392 * t446;
    (t5426, t5427, t5429, t5433, t5435, t5436)
}
