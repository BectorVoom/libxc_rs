//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 613/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk613<F: Float>(t5192: F, t6981: F, t5182: F, t1894: F, t2063: F, t5185: F) -> (F, F, F) {
    let t6982 = t5192 * t6981;
    let t6983 = t5182 * t6982;
    let t6985 = t2063 * t1894;
    let t6986 = t5185 * t6985;
    (t6982, t6983, t6986)
}
