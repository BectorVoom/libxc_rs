//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 861/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk861<F: Float>(t236: F, t495: F, t7230: F, t9210: F, t9211: F, t2145: F, t27: F, t5249: F, t649: F, t34847: F, t9118: F, t16156: F, t9111: F) -> (F, F, F, F) {
    let t39224 = t7230 * t9210 * t236 * t9211 * t495;
    let t39228 = t2145 * t27 * t649 * t5249;
    let t39231 = t34847 * t9118;
    let t39233 = t16156 * t9111;
    (t39224, t39228, t39231, t39233)
}
