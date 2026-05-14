//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1256/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1256<F: Float>(t1100: F, t15627: F, t15703: F, t259: F, t281: F, t3366: F, t3372: F, t3435: F, t3441: F, t1136: F, t15722: F, t14293: F, t2240: F, t4169: F, t6239: F, t1458: F, t20917: F) -> (F, F, F, F, F, F, F, F) {
    let t43683 = t15627 * t1100;
    let t43939 = t259 / t15703 / t281;
    let t43982 = t3366 * t3372;
    let t44167 = t3435 * t3441;
    let t44181 = t1136 * t15722;
    let t48680 = t2240 * t14293;
    let t48691 = t6239 * t4169;
    let t48697 = t20917 * t1458;
    (t43683, t43939, t43982, t44167, t44181, t48680, t48691, t48697)
}
