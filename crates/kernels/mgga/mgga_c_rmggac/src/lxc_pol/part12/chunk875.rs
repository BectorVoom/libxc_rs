//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 875/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk875<F: Float>(t1212: F, t1970: F, t209: F, t236: F, t3352: F, t551: F, t1971: F, t5578: F, t495: F, t7230: F, t9210: F, t9211: F) -> (F, F, F) {
    let t39215 = t1970 * t3352 * t236 * t551 * t1212 * t209;
    let t39219 = t1970 * t1971 * t236 * t5578;
    let t39224 = t7230 * t9210 * t236 * t9211 * t495;
    (t39215, t39219, t39224)
}
