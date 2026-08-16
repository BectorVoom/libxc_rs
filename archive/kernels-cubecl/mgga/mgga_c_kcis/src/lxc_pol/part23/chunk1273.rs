//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1273/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1273<F: Float>(t28453: F, t4142: F, t1394: F, t15925: F, t7923: F, t1464: F, t15911: F, t27387: F, t3245: F, t8171: F, t8179: F, t15819: F, t27331: F, t303: F) -> (F, F, F, F, F, F, F) {
    let t98794 = t4142 * t28453;
    let t98795 = F::cast_from(0.14739506172839506172e-2_f64) * t98794;
    let t98797 = t1394 * t7923 * t15925;
    let t98800 = t1464 * t27387 * t15911;
    let t98804 = t3245 * t8171;
    let t98806 = t3245 * t8179;
    let t98809 = t303 * t27331 * t15819;
    (t98794, t98795, t98797, t98800, t98804, t98806, t98809)
}
