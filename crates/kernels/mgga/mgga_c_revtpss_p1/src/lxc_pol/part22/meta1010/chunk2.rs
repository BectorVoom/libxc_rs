//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3465/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3465<F: Float>(t20050: F, t3106: F, t1063: F, t247: F, t42447: F, t6092: F, t11921: F, t15716: F, t19456: F, t11656: F, t15728: F, t15834: F, t15850: F, t16190: F, t16205: F, t19677: F, t19819: F, t19944: F, t3116: F, t4808: F, t4834: F, t4837: F, t4869: F, t54982: F, t54988: F, t64647: F, t64772: F, t64831: F) -> F {
    let t65288 = t3106 * t20050;
    let t65292 = t1063 * t247 * t42447 * t6092;
    let t65298 = t15716 * t247 * t11921 * t19456;
    let t65316 = F::cast_from(0.85748036236139473944e-3_f64) * t4837 * t247 * t3116 * t64831 - F::cast_from(0.12862205435420921092e-2_f64) * t15716 * t247 * t3116 * t64772 - F::cast_from(0.16937883700965822014e-2_f64) * t65288 - F::cast_from(0.52930886565518193793e-4_f64) * t65292 - F::cast_from(0.91464571985215438873e-2_f64) * t15728 * t19944 - F::cast_from(0.17149607247227894789e-2_f64) * t65298 + F::cast_from(0.51448821741683684368e-2_f64) * t54982 * t247 * t3116 * t64647 + F::cast_from(0.13719685797782315831e-1_f64) * t54988 * t19819 + F::cast_from(0.15244095330869239812e-2_f64) * t11656 * t19677 - F::cast_from(0.45732285992607719436e-2_f64) * t16190 * t4869 + F::cast_from(0.95275595817932748826e-3_f64) * t15850 * t4808 + F::cast_from(0.95275595817932748826e-3_f64) * t4834 * t15834 + F::cast_from(0.47637797908966374413e-3_f64) * t4834 * t16205;
    t65316
}
