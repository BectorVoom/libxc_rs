//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2931/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2931(t11150: f64, t27531: f64, t15908: f64, t999: f64, t1042: f64, t1053: f64, t11804: f64, t15716: f64, t15887: f64, t15907: f64, t1663: f64, t247: f64, t3116: f64, t3117: f64, t3230: f64, t375: f64, t42967: f64, t43105: f64, t4788: f64, t4797: f64, t4837: f64, t51959: f64, t53192: f64, t53318: f64, t53320: f64, t53322: f64, t53326: f64, t53328: f64) -> (f64, f64) {
    let t53332 = t27531 * t11150;
    let t53340 = t15908 * t999;
    let t53351 = -0.34299214494455789577e-2_f64 * t15887 * t1053 * t375 + 0.21722835846488666732e-1_f64 * t4797 * t3230 * t375 - t53318 + 7.0_f64 / 216.0_f64 * t53320 * t53322 * t51959 + 0.63517063878621832551e-4_f64 * t53326 + t53320 * t53328 * t51959 / 16.0_f64 - t53320 * t53332 * t51959 / 12.0_f64 + 0.12862205435420921092e-2_f64 * t4837 * t247 * t3116 * t53192 + 0.38586616306262763275e-2_f64 * t43105 * t3117 * t15907 * t53340 - 0.45732285992607719436e-2_f64 * t42967 * t4788 - 0.38586616306262763276e-2_f64 * t15716 * t1042 * t1663 * t11804;
    (t53340, t53351)
}
