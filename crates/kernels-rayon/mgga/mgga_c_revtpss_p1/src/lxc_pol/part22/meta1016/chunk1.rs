//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3509/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3509(t11922: f64, t15906: f64, t19753: f64, t1011: f64, t1012: f64, t1015: f64, t11687: f64, t15689: f64, t15691: f64, t19501: f64, t19982: f64, t19993: f64, t19998: f64, t23898: f64, t3117: f64, t42804: f64, t43050: f64, t43051: f64, t54014: f64, t54036: f64, t54039: f64, t54042: f64, t54047: f64, t54166: f64, t54801: f64, t55137: f64, t60754: f64, t6266: f64) -> f64 {
    let t66288 = t15906 * t11922 * t19753;
    let t66294 = -0.20325460441158986416e-2_f64 * t54014 - 0.3811023832717309953e-3_f64 * t54036 - 0.28582678745379824648e-3_f64 * t15689 * t15691 * t11687 * t6266 - 0.17149607247227894789e-2_f64 * t54801 * t15691 * t42804 * t23898 - 0.3811023832717309953e-3_f64 * t54039 - 0.19055119163586549765e-3_f64 * t54042 - 0.31758531939310916275e-3_f64 * t54047 - 0.11433071498151929859e-2_f64 * t54166 * t19993 + 0.11433071498151929859e-2_f64 * t55137 * t19998 + 0.95275595817932748826e-3_f64 * t54166 * t19982 + t1011 * t1012 * t1015 * t60754 / 288.0_f64 - 0.1714960724722789479e-2_f64 * t66288 + 0.85748036236139473944e-3_f64 * t43050 * t3117 * t19501 * t43051;
    t66294
}
