//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1202/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1202(t20868: f64, t20888: f64, t664: f64, t684: f64, t1854: f64, t2743: f64, t1857: f64, t1070: f64, t5801: f64, t5805: f64, t1084: f64, t17650: f64) -> (f64, f64, f64, f64) {
    let t20892 = 1.0_f64 * t664 * (t20868 + t20888) * t684;
    let t20893 = t2743 * t1854;
    let t20895 = 6.0_f64 * t20893 * t1857;
    let t20896 = t1070 * t5801;
    let t20898 = 0.51726012919273400301e3_f64 * t20896 * t5805;
    let t20900 = 1.0_f64 * t17650 * t1084;
    (t20892, t20895, t20898, t20900)
}
