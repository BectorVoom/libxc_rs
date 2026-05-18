//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 537/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk537<F: Float>(t3669: F, t381: F, t390: F, t1015: F, t144: F, t377: F, t951: F, t409: F, t1032: F, t1113: F, t1108: F, t360: F, t879: F) -> (F, F, F, F, F, F, F) {
    let t3700 = t381 * t3669;
    let t3702 = F::new(0.17006693853500995666e-1) * t3700 * t390;
    let t3706 = F::new(1.0) / t1015 / t144;
    let t3740 = t377 * t951;
    let t3741 = t3740 * t409;
    let t3745 = t1032 * t1113;
    let t3752 = t1032 * t1108;
    let t3754 = t879 * t360;
    (t3702, t3706, t3740, t3741, t3745, t3752, t3754)
}
