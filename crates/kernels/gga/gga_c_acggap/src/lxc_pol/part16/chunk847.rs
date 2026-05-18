//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 847/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk847<F: Float>(t157: F, t406: F, t864: F, t1487: F, t435: F, t361: F, t171: F, t3300: F, t6576: F, t814: F, t1454: F, t322: F) -> (F, F, F, F, F, F) {
    let t15758 = t864 * t406 * t157;
    let t15995 = t435 * t1487;
    let t17752 = t361 * t435;
    let t17912 = t171 * t3300;
    let t19418 = t6576 * t814;
    let t20311 = t1454 * t322;
    (t15758, t15995, t17752, t17912, t19418, t20311)
}
