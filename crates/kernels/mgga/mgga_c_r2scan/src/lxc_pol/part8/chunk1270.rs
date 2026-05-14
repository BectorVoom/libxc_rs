//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1270/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1270<F: Float>(t26088: F, t8139: F, t19890: F, t6085: F, t9242: F, t2294: F, t2582: F, t8779: F, t481: F, t9235: F, t22980: F, t1604: F, t2526: F, t2841: F, t6243: F, t3087: F, t6240: F) -> (F, F, F, F, F, F, F, F) {
    let t29475 = t26088 * t8139;
    let t29478 = t6085 * t19890 * t9242;
    let t29487 = t2582 * t2294 * t8779;
    let t29496 = t9235 * t481;
    let t29497 = t22980 * t29496;
    let t29498 = t1604 * t29497;
    let t29500 = t2841 * t2526;
    let t29501 = t6243 * t29500;
    let t29502 = t1604 * t29501;
    let t29515 = t6240 * t3087;
    (t29475, t29478, t29487, t29497, t29498, t29501, t29502, t29515)
}
