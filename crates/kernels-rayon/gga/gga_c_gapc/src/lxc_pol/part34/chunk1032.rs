//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1032/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1032(t11905: f64, t7595: f64, t8135: f64, t11356: f64, t3402: f64, t9934: f64, t1084: f64, t9865: f64, t291: f64, t8448: f64, t1971: f64, t9846: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11906 = t11905 * t7595;
    let t11908 = t11905 * t8135;
    let t11910 = t3402 * t11356;
    let t11911 = t11910 * t9934;
    let t11913 = t1084 * t11356;
    let t11914 = t11913 * t9865;
    let t11916 = t8448 * t291;
    let t11917 = t1971 * t11916;
    let t11918 = t1084 * t11917;
    let t11919 = t11918 * t9846;
    (t11906, t11908, t11910, t11911, t11913, t11914, t11917, t11918, t11919)
}
