//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1016/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1016<F: Float>(t11880: F, t263: F, t2938: F, t826: F, t31689: F, t3363: F, t3358: F, t9673: F, t37031: F, t9650: F, t11036: F, t9653: F, t9657: F, t1070: F, t31764: F, t2928: F, t37028: F) -> (F, F, F, F, F, F, F, F) {
    let t42524 = t11880 * t263 * t2938 * t826;
    let t42526 = t31689 * t3363;
    let t42528 = t3358 * t9673;
    let t42530 = t37031 * t9650;
    let t42532 = t11036 * t9653;
    let t42534 = t11036 * t9657;
    let t42536 = t31764 * t1070;
    let t42539 = t37028 * t2928;
    (t42524, t42526, t42528, t42530, t42532, t42534, t42536, t42539)
}
