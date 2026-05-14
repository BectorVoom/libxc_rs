//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1057/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1057<F: Float>(t10466: F, t283: F, t990: F, t26684: F, t61287: F, t14400: F, t2811: F, t1000: F, t1245: F, t1009: F, t9494: F, t44575: F, t7703: F, t7705: F, t9372: F, t1071: F) -> (F, F, F, F, F, F, F, F) {
    let t93366 = t10466 * t283 * t990;
    let t93425 = t26684 * t61287;
    let t93426 = t14400 * t2811;
    let t93435 = t1245 * t1000;
    let t93463 = t1009 * t9494;
    let t93471 = t7703 * t44575 * t7705;
    let t93485 = t1009 * t9372;
    let t93508 = t2811 * t1071;
    (t93366, t93425, t93426, t93435, t93463, t93471, t93485, t93508)
}
