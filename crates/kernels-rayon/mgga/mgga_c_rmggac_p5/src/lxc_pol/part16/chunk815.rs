//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 815/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk815(t16156: f64, t9051: f64, t36343: f64, t9147: f64, t1620: f64, t1986: f64, t7487: f64, t8343: f64, t8358: f64, t8362: f64, t2001: f64, t2281: f64, t326: f64, t333: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40062 = t16156 * t9051;
    let t40075 = t36343 * t9147;
    let t40081 = t1986 * t1620;
    let t40084 = t7487 * t8343;
    let t40086 = t7487 * t8358;
    let t40088 = t7487 * t8362;
    let t40092 = t2001 * t326 * t2281 * t333;
    (t40062, t40075, t40081, t40084, t40086, t40088, t40092)
}
