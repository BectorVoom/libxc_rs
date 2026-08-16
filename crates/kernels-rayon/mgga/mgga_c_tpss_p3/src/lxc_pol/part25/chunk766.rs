//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 766/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk766(t3001: f64, t5198: f64, t1089: f64, t1101: f64, t4579: f64, t926: f64, t3038: f64, t4573: f64, t3033: f64, t1098: f64, t1558: f64, t1564: f64, t3027: f64, t3089: f64, t4212: f64, t4217: f64, t4239: f64, t4258: f64, t4261: f64, t4276: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5199 = t5198 * t3001;
    let t5201 = 0.17315859105681463759e2_f64 * t1089 * t5199;
    let t5206 = t1101 * t4579;
    let t5207 = t926 * t5206;
    let t5210 = t3038 * t4573;
    let t5211 = t926 * t5210;
    let t5214 = t3033 * t4573;
    let t5215 = t926 * t5214;
    let t5222 = -t3027 - t4258 * t1564 / 288.0_f64 + t4212 * t1558 / 54.0_f64 - t1098 * t5207 / 288.0_f64 - t1098 * t5211 / 144.0_f64 + t1098 * t5215 / 216.0_f64 - t3089 - t4261 / 432.0_f64 - t4217 / 432.0_f64 - t4276 / 3456.0_f64 + t4239 / 2304.0_f64;
    (t5199, t5201, t5206, t5210, t5214, t5222)
}
