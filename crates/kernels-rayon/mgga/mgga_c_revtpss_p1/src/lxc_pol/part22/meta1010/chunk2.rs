//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3465/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3465(t20050: f64, t3106: f64, t1063: f64, t247: f64, t42447: f64, t6092: f64, t11921: f64, t15716: f64, t19456: f64, t11656: f64, t15728: f64, t15834: f64, t15850: f64, t16190: f64, t16205: f64, t19677: f64, t19819: f64, t19944: f64, t3116: f64, t4808: f64, t4834: f64, t4837: f64, t4869: f64, t54982: f64, t54988: f64, t64647: f64, t64772: f64, t64831: f64) -> f64 {
    let t65288 = t3106 * t20050;
    let t65292 = t1063 * t247 * t42447 * t6092;
    let t65298 = t15716 * t247 * t11921 * t19456;
    let t65316 = 0.85748036236139473944e-3_f64 * t4837 * t247 * t3116 * t64831 - 0.12862205435420921092e-2_f64 * t15716 * t247 * t3116 * t64772 - 0.16937883700965822014e-2_f64 * t65288 - 0.52930886565518193793e-4_f64 * t65292 - 0.91464571985215438873e-2_f64 * t15728 * t19944 - 0.17149607247227894789e-2_f64 * t65298 + 0.51448821741683684368e-2_f64 * t54982 * t247 * t3116 * t64647 + 0.13719685797782315831e-1_f64 * t54988 * t19819 + 0.15244095330869239812e-2_f64 * t11656 * t19677 - 0.45732285992607719436e-2_f64 * t16190 * t4869 + 0.95275595817932748826e-3_f64 * t15850 * t4808 + 0.95275595817932748826e-3_f64 * t4834 * t15834 + 0.47637797908966374413e-3_f64 * t4834 * t16205;
    t65316
}
