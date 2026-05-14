//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 899/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk899<F: Float>(t1971: F, t2144: F, t3351: F, t45622: F, t7720: F, t9817: F, t39277: F, t9046: F, t17859: F, t8812: F, t2289: F, t38472: F, t46075: F, t903: F, t2185: F, t678: F, t9825: F) -> (F, F, F, F, F, F, F) {
    let t47528 = t3351 * t1971 * t2144 * t45622;
    let t47530 = t7720 * t9817;
    let t47532 = t39277 * t9046;
    let t47534 = t17859 * t8812;
    let t47536 = t38472 * t2289;
    let t47538 = t903 * t46075;
    let t47541 = t9825 * t2185 * t678;
    (t47528, t47530, t47532, t47534, t47536, t47538, t47541)
}
