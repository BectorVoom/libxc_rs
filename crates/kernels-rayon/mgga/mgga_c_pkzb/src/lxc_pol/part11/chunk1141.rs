//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1141/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1141(t1898: f64, t3519: f64, t1901: f64, t9389: f64, t1854: f64, t713: f64, t9462: f64, t1976: f64, t3586: f64, t1954: f64, t694: f64, t9515: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25832 = t3519 * t1898;
    let t25873 = t9389 * t1901;
    let t25908 = t3519 * t1854;
    let t26048 = t9462 * t713;
    let t26053 = t3586 * t1976;
    let t26062 = t3586 * t1954;
    let t26065 = t9515 * t694;
    (t25832, t25873, t25908, t26048, t26053, t26062, t26065)
}
