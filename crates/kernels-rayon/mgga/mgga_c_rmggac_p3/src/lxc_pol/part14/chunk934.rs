//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 934/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk934(t40075: f64, t1620: f64, t1986: f64, t7720: f64, t7487: f64, t8343: f64, t8358: f64, t8362: f64, t2001: f64, t2281: f64, t326: f64, t333: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40076 = 0.24829349937757072982e-4_f64 * t40075;
    let t40081 = t1986 * t1620;
    let t40082 = t7720 * t40081;
    let t40084 = t7487 * t8343;
    let t40085 = 0.19211284388664477842e-2_f64 * t40084;
    let t40086 = t7487 * t8358;
    let t40087 = 0.19211284388664477842e-2_f64 * t40086;
    let t40088 = t7487 * t8362;
    let t40089 = 0.19211284388664477842e-2_f64 * t40088;
    let t40092 = t2001 * t326 * t2281 * t333;
    (t40076, t40082, t40085, t40087, t40089, t40092)
}
