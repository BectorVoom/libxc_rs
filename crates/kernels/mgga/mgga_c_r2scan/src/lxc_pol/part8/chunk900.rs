//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 900/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk900<F: Float>(t6069: F, t7418: F, t2605: F, t6407: F, t2608: F, t6398: F, t2147: F, t2562: F, t481: F, t2148: F, t6165: F, t2567: F, t560: F, t1632: F, t2531: F, t551: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8149 = t6069 * t7418;
    let t8151 = t6407 * t2605;
    let t8153 = t6398 * t2608;
    let t8154 = t2147 * t8153;
    let t8156 = t2562 * t481;
    let t8157 = t2148 * t8156;
    let t8158 = t6165 * t8157;
    let t8160 = t2567 * t560;
    let t8161 = t2148 * t8160;
    let t8163 = 0.34930954652346593434e-1 * t6165 * t8161;
    let t8165 = t551 * t1632 * t2531;
    (t8149, t8151, t8153, t8154, t8156, t8157, t8158, t8160, t8161, t8163, t8165)
}
