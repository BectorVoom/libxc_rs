//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1118/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1118(t1072: f64, t1081: f64, t15351: f64, t1089: f64, t5301: f64, t9519: f64, t4063: f64, t4105: f64, t5082: f64, t9507: f64, t2857: f64, t5114: f64) -> (f64, f64, f64, f64, f64) {
    let t15353 = t1072 * t15351 * t1081;
    let t15355 = 0.5848223622634646207e0_f64 * t1089 * t15353;
    let t15356 = t5301 * t9519;
    let t15361 = 2.0_f64 * t4063 * t4105;
    let t15363 = 2.0_f64 * t9507 * t5082;
    let t15365 = 1.0_f64 * t2857 * t5114;
    (t15355, t15356, t15361, t15363, t15365)
}
