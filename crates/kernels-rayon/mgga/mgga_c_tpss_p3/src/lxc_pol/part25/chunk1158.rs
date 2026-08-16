//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1158/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1158(t1139: f64, t16012: f64, t1136: f64, t1149: f64, t12557: f64, t1587: f64, t15931: f64, t15933: f64, t15944: f64, t15948: f64, t15953: f64, t3113: f64, t4296: f64, t4300: f64, t4323: f64, t473: f64, t5276: f64, t5295: f64) -> f64 {
    let t16013 = t1139 * t16012;
    let t16015 = -6.0_f64 * t1136 * t15944 + 4.0_f64 * t1136 * t15948 + 2.0_f64 * t1136 * t15953 - t1136 * t16013 - t1149 * t15933 - 2.0_f64 * t12557 * t1587 + t15931 * t473 + 2.0_f64 * t3113 * t5276 - t3113 * t5295 + 4.0_f64 * t4296 * t4300 - 2.0_f64 * t4296 * t4323;
    t16015
}
