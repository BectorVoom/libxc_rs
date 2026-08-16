//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1149/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1149(t1005: f64, t6081: f64, t1856: f64, t3228: f64, t1008: f64, t5811: f64, t5816: f64, t1089: f64, t1095: f64, t12762: f64, t12770: f64, t1298: f64, t1524: f64, t15675: f64, t15710: f64, t15714: f64, t4099: f64, t418: f64, t4838: f64, t495: f64, t513: f64) -> f64 {
    let t20672 = t1005 * t6081;
    let t20693 = t3228 * t1856;
    let t20695 = t1008 * t5811;
    let t20697 = t1008 * t5816;
    let t20699 = -0.51448821741683684367e-2_f64 * t15675 + 0.85748036236139473944e-3_f64 * t20672 - 0.12862205435420921092e-2_f64 * t12762 - 0.20007875121765877254e-2_f64 * t12770 + 0.68598428988911579156e-2_f64 * t15710 + 0.34299214494455789578e-2_f64 * t15714 + 0.34299214494455789578e-2_f64 * t418 * t1089 * t1095 * t4099 * t513 + 0.68598428988911579156e-2_f64 * t418 * t1089 * t1095 * t1298 * t1524 + 0.34299214494455789578e-2_f64 * t418 * t1089 * t1095 * t495 * t4838 - 0.17149607247227894789e-2_f64 * t20693 - 0.34299214494455789578e-2_f64 * t20695 - 0.34299214494455789578e-2_f64 * t20697;
    t20699
}
