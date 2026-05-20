//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2989/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2989<F: Float>(t11922: F, t15921: F, t3115: F, t1086: F, t15669: F, t3090: F, t43347: F, t53668: F, t16163: F, t3124: F, t11247: F, t11689: F, t11693: F, t11930: F, t15193: F, t15917: F, t16017: F, t16022: F, t16049: F, t16128: F, t19738: F, t19741: F, t3091: F, t3092: F, t3117: F, t42816: F, t42872: F, t4786: F, t53670: F, t54089: F) -> F {
    let t54497 = t3115 * t11922 * t15921;
    let t54500 = t15669 * t1086 * t3090;
    let t54509 = t43347 * t53668;
    let t54521 = t3124 * t16163;
    let t54526 = -F::cast_from(0.85748036236139473944e-3_f64) * t54497 + F::cast_from(0.12862205435420921092e-2_f64) * t54500 * t11930 + F::cast_from(0.34299214494455789577e-2_f64) * t16049 * t16022 + F::cast_from(0.12862205435420921092e-2_f64) * t19738 * t11689 - F::cast_from(0.64311027177104605458e-3_f64) * t19741 * t11693 + F::cast_from(0.51448821741683684368e-2_f64) * t54509 * t3117 * t53670 * t42872 * t11247 - F::cast_from(0.14291339372689912324e-2_f64) * t54089 * t16128 + F::cast_from(0.42874018118069736972e-3_f64) * t3091 * t3092 * t15193 * t4786 + F::cast_from(0.85748036236139473944e-3_f64) * t54521 + F::cast_from(0.28582678745379824648e-3_f64) * t42816 - F::cast_from(0.12862205435420921092e-2_f64) * t15917 * t16017;
    t54526
}
