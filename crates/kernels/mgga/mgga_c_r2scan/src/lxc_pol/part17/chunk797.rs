//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 797/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk797<F: Float>(t551: F, t552: F, t9124: F, t9129: F, t8692: F, t2719: F, t910: F, t2526: F, t938: F, t1632: F, t3056: F, t574: F, t1600: F, t3064: F, t3087: F, t3053: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9190 = t551 * t552 * t9124;
    let t9194 = t551 * t552 * t9129;
    let t9202 = t551 * t552 * t8692;
    let t9207 = t2719 * t910;
    let t9209 = t551 * t552 * t9207;
    let t9212 = t938 * t2526;
    let t9214 = t551 * t552 * t9212;
    let t9218 = t551 * t1632 * t3056;
    let t9219 = t574 * t9218;
    let t9221 = t1600 * t3064;
    let t9223 = t1600 * t3087;
    let t9226 = t551 * t1632 * t3053;
    (t9190, t9194, t9202, t9209, t9214, t9219, t9221, t9223, t9226)
}
