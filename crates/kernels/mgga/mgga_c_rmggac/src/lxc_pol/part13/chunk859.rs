//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 859/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk859<F: Float>(t3351: F, t515: F, t5260: F, t9188: F, t1594: F, t1986: F, t7720: F, t1627: F, t3352: F, t495: F, t511: F, t7230: F) -> (F, F, F) {
    let t39197 = t3351 * t9188 * t515 * t5260;
    let t39199 = t1986 * t1594;
    let t39200 = t7720 * t39199;
    let t39205 = t7230 * t3352 * t511 * t1627 * t495;
    (t39197, t39200, t39205)
}
