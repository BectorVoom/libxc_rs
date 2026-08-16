//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1026/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1026(t10040: f64, t7720: f64, t2310: f64, t38351: f64, t38355: f64, t8571: f64, t8597: f64, t17859: f64, t8504: f64, t8508: f64, t8808: f64, t1971: f64, t3351: f64, t6558: f64, t7262: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46873 = t7720 * t10040;
    let t46875 = t38351 * t2310;
    let t46877 = t38355 * t2310;
    let t46879 = t8571 * t8597;
    let t46881 = t17859 * t8504;
    let t46883 = t17859 * t8508;
    let t46885 = t17859 * t8808;
    let t46889 = t3351 * t1971 * t7262 * t6558;
    (t46873, t46875, t46877, t46879, t46881, t46883, t46885, t46889)
}
