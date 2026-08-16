//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3021/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3021(t3133: f64, t4772: f64, t3105: f64, t4797: f64, t15725: f64, t15827: f64, t11921: f64, t16152: f64, t247: f64, t4837: f64, t1045: f64, t1068: f64, t11859: f64, t15728: f64, t15839: f64, t15895: f64, t15899: f64, t16154: f64, t1675: f64, t3115: f64, t3117: f64, t3155: f64, t42643: f64, t42675: f64, t42830: f64, t43091: f64, t43121: f64, t4907: f64, t53792: f64) -> (f64, f64) {
    let t55345 = t4772 * t3133;
    let t55356 = t4797 * t3105;
    let t55361 = t15725 * t15827;
    let t55367 = t4837 * t247 * t11921 * t16152;
    let t55371 = -0.34299214494455789577e-2_f64 * t42675 * t15899 - 0.12862205435420921092e-2_f64 * t42643 * t15895 + 0.64311027177104605458e-3_f64 * t42830 * t15899 - 0.64311027177104605458e-3_f64 * t3115 * t3117 * t55345 * t1045 - 0.12862205435420921092e-2_f64 * t11859 * t3117 * t53792 * t3155 + 0.68598428988911579154e-2_f64 * t43121 * t4907 - 0.45732285992607719436e-2_f64 * t55356 * t1068 + 0.14481890564325777822e-1_f64 * t43091 * t1675 + 0.17149607247227894789e-2_f64 * t55361 - 0.13719685797782315831e-1_f64 * t15728 * t16154 + 0.17149607247227894789e-2_f64 * t55367 - 0.68598428988911579154e-2_f64 * t15728 * t15839;
    (t55345, t55371)
}
