//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2535/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2535<F: Float>(t19856: F, t225: F, t366: F, t15696: F, t4782: F, t4787: F, t1058: F, t6318: F, t1053: F, t6317: F, t4786: F, t6096: F) -> (F, F, F, F, F, F, F) {
    let t19857 = t19856 * t225;
    let t19858 = t19857 * t366;
    let t19861 = t15696 * t4782;
    let t19864 = t15696 * t4787;
    let t19867 = t6318 * t1058;
    let t19869 = t6317 * t1053;
    let t19872 = t6096 * t4786;
    (t19857, t19858, t19861, t19864, t19867, t19869, t19872)
}
