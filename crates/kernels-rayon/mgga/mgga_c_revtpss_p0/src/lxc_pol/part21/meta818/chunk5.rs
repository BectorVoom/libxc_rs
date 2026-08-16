//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3016/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3016(t1043: f64, t15648: f64, t11922: f64, t16039: f64, t3115: f64, t11859: f64, t15610: f64, t1032: f64, t1040: f64, t15886: f64, t1011: f64, t1012: f64, t1015: f64, t1045: f64, t1047: f64, t11173: f64, t11231: f64, t16123: f64, t3092: f64, t3117: f64, t3241: f64, t357: f64, t43242: f64, t43266: f64, t43277: f64, t4573: f64, t4781: f64, t49889: f64, t55011: f64) -> (f64, f64) {
    let t55165 = t15648 * t1043;
    let t55171 = t3115 * t11922 * t16039;
    let t55182 = t11859 * t11922 * t15610;
    let t55195 = t15886 * t1032 * t1040;
    let t55198 = -0.64311027177104605458e-3_f64 * t3115 * t3117 * t55165 * t1045 - 0.85748036236139473944e-3_f64 * t55171 - 0.19055119163586549765e-3_f64 * t43242 - 0.28582678745379824648e-3_f64 * t43266 - t3241 * t16123 / 36.0_f64 + t1011 * t1012 * t1015 * t49889 / 288.0_f64 - 0.17149607247227894789e-2_f64 * t55182 - 0.42874018118069736972e-3_f64 * t43277 - 0.21437009059034868486e-3_f64 * t3115 * t3117 * t4781 * t357 * t11173 + 0.25724410870841842183e-2_f64 * t55011 * t3092 * t4573 * t11231 + 0.64311027177104605458e-3_f64 * t55195 * t1047;
    (t55165, t55198)
}
