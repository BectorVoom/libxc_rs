//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2931/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2931<F: Float>(t11150: F, t27531: F, t15908: F, t999: F, t1042: F, t1053: F, t11804: F, t15716: F, t15887: F, t15907: F, t1663: F, t247: F, t3116: F, t3117: F, t3230: F, t375: F, t42967: F, t43105: F, t4788: F, t4797: F, t4837: F, t51959: F, t53192: F, t53318: F, t53320: F, t53322: F, t53326: F, t53328: F) -> (F, F) {
    let t53332 = t27531 * t11150;
    let t53340 = t15908 * t999;
    let t53351 = -F::cast_from(0.34299214494455789577e-2_f64) * t15887 * t1053 * t375 + F::cast_from(0.21722835846488666732e-1_f64) * t4797 * t3230 * t375 - t53318 + F::new(7.0) / F::new(216.0) * t53320 * t53322 * t51959 + F::cast_from(0.63517063878621832551e-4_f64) * t53326 + t53320 * t53328 * t51959 / F::new(16.0) - t53320 * t53332 * t51959 / F::new(12.0) + F::cast_from(0.12862205435420921092e-2_f64) * t4837 * t247 * t3116 * t53192 + F::cast_from(0.38586616306262763275e-2_f64) * t43105 * t3117 * t15907 * t53340 - F::cast_from(0.45732285992607719436e-2_f64) * t42967 * t4788 - F::cast_from(0.38586616306262763276e-2_f64) * t15716 * t1042 * t1663 * t11804;
    (t53340, t53351)
}
