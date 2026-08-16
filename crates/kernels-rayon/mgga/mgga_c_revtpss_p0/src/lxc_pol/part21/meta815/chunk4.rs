//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2989/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2989(t11922: f64, t15921: f64, t3115: f64, t1086: f64, t15669: f64, t3090: f64, t43347: f64, t53668: f64, t16163: f64, t3124: f64, t11247: f64, t11689: f64, t11693: f64, t11930: f64, t15193: f64, t15917: f64, t16017: f64, t16022: f64, t16049: f64, t16128: f64, t19738: f64, t19741: f64, t3091: f64, t3092: f64, t3117: f64, t42816: f64, t42872: f64, t4786: f64, t53670: f64, t54089: f64) -> f64 {
    let t54497 = t3115 * t11922 * t15921;
    let t54500 = t15669 * t1086 * t3090;
    let t54509 = t43347 * t53668;
    let t54521 = t3124 * t16163;
    let t54526 = -0.85748036236139473944e-3_f64 * t54497 + 0.12862205435420921092e-2_f64 * t54500 * t11930 + 0.34299214494455789577e-2_f64 * t16049 * t16022 + 0.12862205435420921092e-2_f64 * t19738 * t11689 - 0.64311027177104605458e-3_f64 * t19741 * t11693 + 0.51448821741683684368e-2_f64 * t54509 * t3117 * t53670 * t42872 * t11247 - 0.14291339372689912324e-2_f64 * t54089 * t16128 + 0.42874018118069736972e-3_f64 * t3091 * t3092 * t15193 * t4786 + 0.85748036236139473944e-3_f64 * t54521 + 0.28582678745379824648e-3_f64 * t42816 - 0.12862205435420921092e-2_f64 * t15917 * t16017;
    t54526
}
