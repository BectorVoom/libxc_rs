//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1150/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1150(t10252: f64, t2099: f64, t3235: f64, t2411: f64, t9795: f64, t10189: f64, t2029: f64, t10225: f64, t18657: f64, t2380: f64, t10097: f64, t3185: f64, t926: f64) -> (f64, f64, f64, f64, f64) {
    let t27083 = t3235 * t2099 * t10252;
    let t27085 = t2411 * t9795;
    let t27104 = t10189 * t2029;
    let t27119 = t2380 * t18657 * t10225;
    let t27122 = t3185 * t926 * t10097;
    (t27083, t27085, t27104, t27119, t27122)
}
