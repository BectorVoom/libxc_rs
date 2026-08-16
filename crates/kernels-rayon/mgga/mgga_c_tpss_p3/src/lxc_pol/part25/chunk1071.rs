//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1071/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1071(t4918: f64, t895: f64, t11351: f64, t14447: f64, t14449: f64, t14451: f64, t1449: f64, t14573: f64, t14575: f64, t14578: f64, t14681: f64, t14719: f64, t305: f64, t3860: f64, t3883: f64, t4924: f64, t8906: f64, t905: f64) -> f64 {
    let t14722 = t4918 * t895;
    let t14731 = -0.19751673498613801407e-1_f64 * t14681 - 0.310907e-1_f64 * t14719 * t305 - t14447 + t14449 - t14451 - t14573 - t14575 - t14578 + 0.5848223622634646207e0_f64 * t14722 * t905 + 0.11696447245269292414e1_f64 * t11351 * t1449 + 0.11696447245269292414e1_f64 * t3860 * t3883 - 0.11696447245269292414e1_f64 * t8906 * t4924;
    t14731
}
