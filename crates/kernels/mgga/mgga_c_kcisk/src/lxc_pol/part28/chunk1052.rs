//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1052/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1052<F: Float>(t5278: F, t9043: F, t17969: F, t7299: F, t2567: F, t7405: F, t1935: F, t17056: F, t5320: F, t7431: F, t22283: F, t7316: F, t7315: F, t1936: F, t9054: F, t24001: F, t5290: F) -> (F, F, F, F, F, F, F, F) {
    let t24130 = t5278 * t9043;
    let t24132 = t17969 * t7299;
    let t24134 = t2567 * t7405;
    let t24135 = t1935 * t24134;
    let t24137 = t17056 * t5320;
    let t24138 = t24137 * t7431;
    let t24140 = t7316 * t22283;
    let t24141 = t7315 * t24140;
    let t24143 = t9054 * t1936;
    let t24144 = t1935 * t24143;
    let t24146 = t5290 * t24001;
    (t24130, t24132, t24135, t24138, t24140, t24141, t24144, t24146)
}
