//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1130/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1130(t15266: f64, t4219: f64, t15275: f64, t4223: f64, t15271: f64, t4597: f64, t924: f64, t140: f64, t5210: f64, t1098: f64, t5214: f64, t1095: f64, t5223: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15544 = t4219 * t15266;
    let t15547 = t4223 * t15275;
    let t15550 = t4223 * t15271;
    let t15554 = t4597 * t924;
    let t15557 = t140 * t5210;
    let t15558 = t1098 * t15557;
    let t15560 = t140 * t5214;
    let t15561 = t1098 * t15560;
    let t15564 = t5223 * t1095;
    (t15544, t15547, t15550, t15554, t15558, t15561, t15564)
}
