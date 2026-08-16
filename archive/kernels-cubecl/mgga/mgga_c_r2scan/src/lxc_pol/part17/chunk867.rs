//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 867/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk867<F: Float>(t1600: F, t3064: F, t3087: F, t1632: F, t3053: F, t551: F, t574: F, t2620: F, t2651: F, t3090: F, t1592: F, t133: F, t2892: F) -> (F, F, F, F, F, F) {
    let t9221 = t1600 * t3064;
    let t9223 = t1600 * t3087;
    let t9226 = t551 * t1632 * t3053;
    let t9227 = t574 * t9226;
    let t9229 = t2651 * t2620;
    let t9232 = t551 * t1632 * t3090;
    let t9233 = t1592 * t9232;
    let t9235 = t133 * t2892;
    (t9221, t9223, t9227, t9229, t9233, t9235)
}
