//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1251/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1251(t1020: f64, t27836: f64, t4806: f64, t4548: f64, t19553: f64, t7718: f64, t11068: f64, t29111: f64, t7788: f64, t29107: f64, t3500: f64, t29159: f64, t922: f64, t92693: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t100420 = t1020 * t27836 * t4806;
    let t100423 = t1020 * t27836 * t4548;
    let t100426 = t1020 * t7718 * t19553;
    let t100429 = t7788 * t11068 * t29111;
    let t100432 = t7788 * t3500 * t29107;
    let t100436 = t92693 * t29159 * t922;
    (t100420, t100423, t100426, t100429, t100432, t100436)
}
