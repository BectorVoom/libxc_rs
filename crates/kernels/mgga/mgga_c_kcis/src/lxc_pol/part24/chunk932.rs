//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 932/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk932<F: Float>(t15573: F, t7710: F, t2173: F, t7692: F, t10466: F, t1250: F, t2836: F, t3489: F, t7696: F, t7699: F, t283: F, t3049: F, t990: F) -> (F, F, F, F, F, F, F, F) {
    let t26714 = t15573 * t7710;
    let t26715 = t2173 * t26714;
    let t26717 = t15573 * t7692;
    let t26718 = t2173 * t26717;
    let t26728 = t10466 * t1250;
    let t26739 = t2836 * t3489;
    let t26745 = t7696 * t7699;
    let t26748 = t3049 * t283 * t990;
    (t26714, t26715, t26717, t26718, t26728, t26739, t26745, t26748)
}
