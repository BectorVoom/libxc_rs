//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 753/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk753<F: Float>(t11218: F, t28228: F, t5192: F, t6674: F, t2364: F, t9089: F, t10365: F, t5182: F, t6719: F, t8958: F, t5054: F, t2441: F, t8672: F, t1899: F, t5062: F, t1869: F) -> (F, F, F, F, F) {
    let t28242 = t11218 * t28228;
    let t28243 = t5192 * t28242;
    let t28244 = t6674 * t28243;
    let t28248 = t9089 * t2364;
    let t28249 = t10365 * t28248;
    let t28250 = t5182 * t28249;
    let t28252 = t6719 * t8958;
    let t28253 = t5054 * t28252;
    let t28256 = t8672 * t2441;
    let t28257 = t1899 * t28256;
    let t28258 = t5062 * t28257;
    let t28259 = t1869 * t28258;
    (t28244, t28250, t28253, t28256, t28259)
}
