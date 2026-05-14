//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1294/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1294<F: Float>(t19872: F, t9247: F, t1600: F, t9427: F, t2207: F, t2691: F, t8279: F, t2184: F, t3190: F, t551: F, t6343: F, t2892: F, t6212: F, t20852: F, t6211: F, t26278: F, t7923: F) -> (F, F, F, F, F, F) {
    let t30653 = t19872 * t9247;
    let t30668 = t1600 * t9427;
    let t30676 = t2207 * t8279 * t2691;
    let t30680 = t2184 * t551 * t6343 * t3190;
    let t30691 = t6212 * t2892;
    let t30693 = t20852 * t6211 * t30691;
    let t30765 = t26278 * t7923;
    (t30653, t30668, t30676, t30680, t30693, t30765)
}
