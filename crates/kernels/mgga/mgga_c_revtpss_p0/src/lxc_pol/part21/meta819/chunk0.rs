//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3021/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3021<F: Float>(t3133: F, t4772: F, t3105: F, t4797: F, t15725: F, t15827: F, t11921: F, t16152: F, t247: F, t4837: F, t1045: F, t1068: F, t11859: F, t15728: F, t15839: F, t15895: F, t15899: F, t16154: F, t1675: F, t3115: F, t3117: F, t3155: F, t42643: F, t42675: F, t42830: F, t43091: F, t43121: F, t4907: F, t53792: F) -> (F, F) {
    let t55345 = t4772 * t3133;
    let t55356 = t4797 * t3105;
    let t55361 = t15725 * t15827;
    let t55367 = t4837 * t247 * t11921 * t16152;
    let t55371 = -F::cast_from(0.34299214494455789577e-2_f64) * t42675 * t15899 - F::cast_from(0.12862205435420921092e-2_f64) * t42643 * t15895 + F::cast_from(0.64311027177104605458e-3_f64) * t42830 * t15899 - F::cast_from(0.64311027177104605458e-3_f64) * t3115 * t3117 * t55345 * t1045 - F::cast_from(0.12862205435420921092e-2_f64) * t11859 * t3117 * t53792 * t3155 + F::cast_from(0.68598428988911579154e-2_f64) * t43121 * t4907 - F::cast_from(0.45732285992607719436e-2_f64) * t55356 * t1068 + F::cast_from(0.14481890564325777822e-1_f64) * t43091 * t1675 + F::cast_from(0.17149607247227894789e-2_f64) * t55361 - F::cast_from(0.13719685797782315831e-1_f64) * t15728 * t16154 + F::cast_from(0.17149607247227894789e-2_f64) * t55367 - F::cast_from(0.68598428988911579154e-2_f64) * t15728 * t15839;
    (t55345, t55371)
}
