//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3016/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3016<F: Float>(t1043: F, t15648: F, t11922: F, t16039: F, t3115: F, t11859: F, t15610: F, t1032: F, t1040: F, t15886: F, t1011: F, t1012: F, t1015: F, t1045: F, t1047: F, t11173: F, t11231: F, t16123: F, t3092: F, t3117: F, t3241: F, t357: F, t43242: F, t43266: F, t43277: F, t4573: F, t4781: F, t49889: F, t55011: F) -> (F, F) {
    let t55165 = t15648 * t1043;
    let t55171 = t3115 * t11922 * t16039;
    let t55182 = t11859 * t11922 * t15610;
    let t55195 = t15886 * t1032 * t1040;
    let t55198 = -F::cast_from(0.64311027177104605458e-3_f64) * t3115 * t3117 * t55165 * t1045 - F::cast_from(0.85748036236139473944e-3_f64) * t55171 - F::cast_from(0.19055119163586549765e-3_f64) * t43242 - F::cast_from(0.28582678745379824648e-3_f64) * t43266 - t3241 * t16123 / F::new(36.0) + t1011 * t1012 * t1015 * t49889 / F::new(288.0) - F::cast_from(0.17149607247227894789e-2_f64) * t55182 - F::cast_from(0.42874018118069736972e-3_f64) * t43277 - F::cast_from(0.21437009059034868486e-3_f64) * t3115 * t3117 * t4781 * t357 * t11173 + F::cast_from(0.25724410870841842183e-2_f64) * t55011 * t3092 * t4573 * t11231 + F::cast_from(0.64311027177104605458e-3_f64) * t55195 * t1047;
    (t55165, t55198)
}
