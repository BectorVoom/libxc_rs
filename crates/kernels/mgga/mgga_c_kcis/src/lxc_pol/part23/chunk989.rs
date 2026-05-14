//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 989/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk989<F: Float>(t167: F, t7909: F, t16892: F, t18210: F, t8158: F, t2237: F, t8147: F, t7898: F, t5628: F, t7931: F, t303: F, t1307: F, t28373: F, t3984: F, t5885: F, t5709: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t28419 = t7909 * t167;
    let t28420 = t16892 * t28419;
    let t28423 = t18210 * t8158;
    let t28424 = t2237 * t28423;
    let t28426 = t18210 * t8147;
    let t28427 = t7898 * t28426;
    let t28429 = t7931 * t5628;
    let t28430 = t303 * t28429;
    let t28438 = t28373 * t1307;
    let t28439 = t3984 * t28438;
    let t28442 = t5885 * t1307;
    let t28443 = t5709 * t28442;
    (t28419, t28420, t28423, t28424, t28426, t28427, t28429, t28430, t28438, t28439, t28442, t28443)
}
