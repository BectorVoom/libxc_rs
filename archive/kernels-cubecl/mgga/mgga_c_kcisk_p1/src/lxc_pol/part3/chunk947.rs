//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 947/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk947<F: Float>(t3886: F, t965: F, t12925: F, t1383: F, t12831: F, t3661: F, t12952: F, t457: F, t3894: F, t1384: F, t3119: F, t1399: F, t3123: F) -> (F, F, F, F, F, F, F) {
    let t14014 = t965 * t3886;
    let t14016 = t1383 * t12925;
    let t14019 = t3661 * t12831;
    let t14022 = t457 * t12952;
    let t14025 = t965 * t3894;
    let t14027 = t3119 * t1384;
    let t14029 = t3123 * t1399;
    (t14014, t14016, t14019, t14022, t14025, t14027, t14029)
}
