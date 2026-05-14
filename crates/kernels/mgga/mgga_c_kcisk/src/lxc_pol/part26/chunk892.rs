//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 892/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk892<F: Float>(t20895: F, t13959: F, t6235: F, t1458: F, t6239: F, t2240: F, t4169: F) -> (F, F, F, F, F) {
    let t20896 = 0.14739506172839506172e-2 * t20895;
    let t20897 = t13959 * t6235;
    let t20898 = 0.22109259259259259258e-2 * t20897;
    let t20919 = t6239 * t1458;
    let t20922 = t2240 * t4169;
    (t20896, t20897, t20898, t20919, t20922)
}
