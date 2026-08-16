//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1025/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1025<F: Float>(t16804: F, t252: F, t1492: F, t4265: F, t225: F, t5632: F, t5561: F, t1519: F, t4142: F, t5631: F, t798: F, t5558: F, t852: F) -> (F, F, F, F, F, F, F) {
    let t17083 = t16804 * t252;
    let t17087 = t1492 * t4265;
    let t17090 = t5632 * t225;
    let t17092 = t5561 * t225;
    let t17095 = t4142 * t1519;
    let t17098 = t798 * t5631;
    let t17100 = t5558 * t852;
    (t17083, t17087, t17090, t17092, t17095, t17098, t17100)
}
