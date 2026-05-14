//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1170/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1170<F: Float>(t1762: F, t5206: F, t5594: F, t377: F, t5266: F, t5539: F, t1813: F, t234: F, t5279: F, t5285: F, t5351: F, t21115: F, t5291: F, t720: F, t5296: F, t732: F) -> (F, F, F, F, F, F) {
    let t22292 = 0.11407595979765752406e3 * t1762 * t5206 * t5594;
    let t22296 = 0.33776465721256572866e4 * t1762 * t377 * t5266 * t5539;
    let t22305 = 0.10389515463408878255e3 * t234 * t5279 * t1813;
    let t22308 = 0.56142946777292603589e2 * t234 * t5351 * t5285;
    let t22312 = 0.4155806185363551302e4 * t234 * t5291 * t720 * t21115;
    let t22315 = t732 * t5296;
    (t22292, t22296, t22305, t22308, t22312, t22315)
}
