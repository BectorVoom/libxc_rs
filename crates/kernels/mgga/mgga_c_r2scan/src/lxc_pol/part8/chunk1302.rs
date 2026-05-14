//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1302/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1302<F: Float>(t2207: F, t2837: F, t7402: F, t1610: F, t9418: F, t25196: F, t259: F, t571: F, t6118: F, t9513: F, t19890: F, t6093: F, t9246: F, t6407: F, t9250: F, t26278: F, t8082: F) -> (F, F, F, F, F, F, F) {
    let t31117 = t2207 * t2837 * t7402;
    let t31120 = t2207 * t1610 * t9418;
    let t31131 = t571 * t25196 * t259;
    let t31144 = t6118 * t9513;
    let t31156 = t6093 * t19890 * t9246;
    let t31158 = t6407 * t9250;
    let t31160 = t26278 * t8082;
    (t31117, t31120, t31131, t31144, t31156, t31158, t31160)
}
