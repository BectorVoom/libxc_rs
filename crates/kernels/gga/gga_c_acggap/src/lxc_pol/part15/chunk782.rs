//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 782/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk782<F: Float>(t157: F, t406: F, t864: F, t1487: F, t435: F, t361: F, t171: F, t3300: F, t6576: F, t814: F, t1454: F, t322: F, t1164: F, t5679: F, t174: F, t507: F) -> (F, F, F, F, F, F, F, F) {
    let t15758 = t864 * t406 * t157;
    let t15995 = t435 * t1487;
    let t17752 = t361 * t435;
    let t17912 = t171 * t3300;
    let t19418 = t6576 * t814;
    let t20311 = t1454 * t322;
    let t20417 = t1164 * t5679;
    let t20555 = t507 * t174;
    (t15758, t15995, t17752, t17912, t19418, t20311, t20417, t20555)
}
