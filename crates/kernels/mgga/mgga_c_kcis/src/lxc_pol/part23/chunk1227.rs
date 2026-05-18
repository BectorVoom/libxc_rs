//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1227/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1227<F: Float>(t1307: F, t16681: F, t5709: F, t3805: F, t5885: F, t3797: F, t5701: F, t1464: F, t28338: F, t94216: F, t27376: F, t28392: F) -> (F, F, F, F, F) {
    let t98002 = t5709 * t16681 * t1307;
    let t98006 = t5709 * t5885 * t3805;
    let t98010 = t5701 * t5885 * t3797;
    let t98014 = t1464 * t94216 * t28338;
    let t98016 = t28392 * t27376;
    (t98002, t98006, t98010, t98014, t98016)
}
