//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 783/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk783(t1139: f64, t4322: f64, t1136: f64, t1149: f64, t1587: f64, t3113: f64, t4294: f64, t4296: f64, t4300: f64, t473: f64, t1589: f64, t3154: f64) -> (f64, f64, f64) {
    let t4323 = t1139 * t4322;
    let t4325 = 2.0_f64 * t1136 * t4300 - t1136 * t4323 - t1149 * t4296 - t1587 * t3113 + t4294 * t473;
    let t4329 = t1589 * t3154;
    (t4323, t4325, t4329)
}
