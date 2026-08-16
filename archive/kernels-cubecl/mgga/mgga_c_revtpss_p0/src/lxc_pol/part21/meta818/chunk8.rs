//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3019/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3019<F: Float>(t11988: F, t4834: F, t15731: F, t3124: F, t11933: F, t15794: F, t3115: F, t42793: F, t4911: F, t1062: F, t11231: F, t11637: F, t15139: F, t15782: F, t15957: F, t16043: F, t16052: F, t16089: F, t20094: F, t3092: F, t3117: F, t42359: F, t42410: F, t43288: F, t4839: F, t4892: F, t4894: F, t54909: F, t55011: F) -> F {
    let t55272 = t4834 * t11988;
    let t55279 = t3124 * t15731;
    let t55280 = F::cast_from(0.14291339372689912324e-3_f64) * t55279;
    let t55290 = t11933 * t15794;
    let t55293 = t3115 * t42793 * t4911;
    let t55294 = F::cast_from(0.14291339372689912324e-3_f64) * t55293;
    let t55303 = -F::cast_from(0.95275595817932748825e-4_f64) * t55272 + F::cast_from(0.12862205435420921092e-2_f64) * t42359 * t1062 * t4839 - F::cast_from(0.13719685797782315831e-1_f64) * t16052 * t15782 - t55280 - F::cast_from(0.85748036236139473944e-3_f64) * t43288 - F::cast_from(0.64311027177104605458e-3_f64) * t3115 * t3117 * t15957 * t16043 + F::cast_from(0.12862205435420921092e-2_f64) * t4892 * t3117 * t54909 * t4894 + F::cast_from(0.45732285992607719436e-2_f64) * t55290 + t55294 + F::cast_from(0.19055119163586549765e-2_f64) * t55011 * t42410 * t15139 * t11231 - F::cast_from(0.17149607247227894789e-2_f64) * t16089 * t3092 * t20094 * t11637;
    t55303
}
