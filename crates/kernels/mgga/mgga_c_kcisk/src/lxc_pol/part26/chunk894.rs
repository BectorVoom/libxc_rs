//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 894/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk894<F: Float>(t140: F, t299: F, t6303: F, t4265: F, t6284: F, t15197: F, t6292: F, t18053: F, t6288: F, t6280: F, t2242: F, t306: F, t2253: F, t430: F, t1413: F, t6308: F) -> (F, F, F, F, F, F, F, F) {
    let t21163 = 0.53062222222222222222e-1 * t140 * t299 * t6303;
    let t21168 = 0.35374814814814814814e-1 * t4265 * t6284;
    let t21169 = t15197 * t6292;
    let t21177 = t18053 * t6288;
    let t21180 = 0.5895802469135802469e-1 * t18053 * t6280;
    let t21252 = t2242 * t306;
    let t21256 = t140 * t430 * t2253;
    let t21289 = t6308 * t1413;
    (t21163, t21168, t21169, t21177, t21180, t21252, t21256, t21289)
}
