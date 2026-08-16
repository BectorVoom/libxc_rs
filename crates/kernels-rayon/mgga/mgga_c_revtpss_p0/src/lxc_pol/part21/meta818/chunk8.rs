//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3019/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3019(t11988: f64, t4834: f64, t15731: f64, t3124: f64, t11933: f64, t15794: f64, t3115: f64, t42793: f64, t4911: f64, t1062: f64, t11231: f64, t11637: f64, t15139: f64, t15782: f64, t15957: f64, t16043: f64, t16052: f64, t16089: f64, t20094: f64, t3092: f64, t3117: f64, t42359: f64, t42410: f64, t43288: f64, t4839: f64, t4892: f64, t4894: f64, t54909: f64, t55011: f64) -> f64 {
    let t55272 = t4834 * t11988;
    let t55279 = t3124 * t15731;
    let t55280 = 0.14291339372689912324e-3_f64 * t55279;
    let t55290 = t11933 * t15794;
    let t55293 = t3115 * t42793 * t4911;
    let t55294 = 0.14291339372689912324e-3_f64 * t55293;
    let t55303 = -0.95275595817932748825e-4_f64 * t55272 + 0.12862205435420921092e-2_f64 * t42359 * t1062 * t4839 - 0.13719685797782315831e-1_f64 * t16052 * t15782 - t55280 - 0.85748036236139473944e-3_f64 * t43288 - 0.64311027177104605458e-3_f64 * t3115 * t3117 * t15957 * t16043 + 0.12862205435420921092e-2_f64 * t4892 * t3117 * t54909 * t4894 + 0.45732285992607719436e-2_f64 * t55290 + t55294 + 0.19055119163586549765e-2_f64 * t55011 * t42410 * t15139 * t11231 - 0.17149607247227894789e-2_f64 * t16089 * t3092 * t20094 * t11637;
    t55303
}
