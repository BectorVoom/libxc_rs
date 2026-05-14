//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 906/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk906<F: Float>(t1142: F, t15092: F, t1872: F, t3699: F, t1291: F, t5394: F, t3670: F, t11223: F, t11230: F, t14667: F, t14670: F, t14671: F, t14682: F, t14685: F, t3664: F, t3669: F, t5360: F, t5363: F) -> (F, F, F, F, F) {
    let t15093 = t1142 * t15092;
    let t15095 = t1872 * t3699;
    let t15098 = t5394 * t1291;
    let t15101 = t1872 * t3670;
    let t15108 = 4.0 * t11223 * t5363 - 6.0 * t11230 * t15101 + 2.0 * t15095 * t3669 + 4.0 * t15098 * t3669 - 2.0 * t3664 * t5394 - t3699 * t5360 + t14667 - t14670 + t14671 - t14682 - t14685;
    (t15093, t15095, t15098, t15101, t15108)
}
