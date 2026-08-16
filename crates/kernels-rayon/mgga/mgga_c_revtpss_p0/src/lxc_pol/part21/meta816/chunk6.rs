//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3001/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3001(t11710: f64, t15591: f64, t3091: f64, t16060: f64, t3241: f64, t1011: f64, t140: f64, t16122: f64, t12078: f64, t53740: f64, t11661: f64, t11684: f64, t11696: f64, t11883: f64, t11927: f64, t12131: f64, t15618: f64, t15689: f64, t15691: f64, t15700: f64, t15717: f64, t15957: f64, t16025: f64, t16190: f64, t19980: f64, t3117: f64, t3136: f64, t42316: f64, t42804: f64, t43291: f64, t4786: f64, t4887: f64) -> f64 {
    let t54785 = t3091 * t11710 * t15591;
    let t54792 = t3241 * t16060;
    let t54795 = t1011 * t140 * t16122;
    let t54801 = t12078 * t53740;
    let t54806 = -0.34299214494455789577e-2_f64 * t16190 * t3136 + 0.12862205435420921092e-2_f64 * t11927 * t3117 * t15957 * t16025 - 0.85748036236139473944e-3_f64 * t15618 * t11684 - 0.38586616306262763275e-2_f64 * t43291 * t3117 * t15717 * t4786 + 0.28582678745379824648e-3_f64 * t54785 + 0.71456696863449561621e-3_f64 * t15700 * t19980 * t42316 + 11.0_f64 / 108.0_f64 * t11883 * t4887 - t54792 / 54.0_f64 + t54795 / 288.0_f64 - 0.42874018118069736972e-3_f64 * t15689 * t15691 * t12131 * t11696 - 0.25724410870841842183e-2_f64 * t54801 * t15691 * t42804 * t11661;
    t54806
}
