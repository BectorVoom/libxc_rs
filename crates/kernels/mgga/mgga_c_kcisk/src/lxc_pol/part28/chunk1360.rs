//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1360/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1360<F: Float>(t112192: F, t1636: F, t35095: F, t34072: F, t7218: F, t34153: F, t35085: F, t5074: F, t34045: F, t34073: F, t116368: F, t116370: F, t116372: F, t116380: F, t116394: F, t116423: F, t116426: F, t33056: F, t34154: F, t34182: F, t9652: F) -> (F, F, F, F) {
    let t121314 = t112192 * t35095 * t1636;
    let t121323 = t34072 * t7218;
    let t121326 = t34153 * t7218;
    let t121329 = t5074 * t35085;
    let t121332 = t34073 * t34045;
    let t121335 = -0.26805555555555555557e-2 * t33056 * t121314 - t116368 - t116370 + 0.46296296296296296296e-2 * t116372 - 0.20833333333333333334e-1 * t34073 * t34182 - 0.22109259259259259259e-2 * t116380 - 0.80416666666666666669e-2 * t34154 * t34182 + t116394 - 0.55555555555555555558e-1 * t121323 * t9652 - 0.21444444444444444445e-1 * t121326 * t9652 - 0.22109259259259259259e-2 * t121329 + 0.22109259259259259259e-2 * t116423 + 0.69444444444444444447e-2 * t121332 + 0.46296296296296296296e-2 * t116426;
    (t121314, t121323, t121329, t121335)
}
