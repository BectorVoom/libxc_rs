//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1271/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1271<F: Float>(t1567: F, t3177: F, t1632: F, t2184: F, t551: F, t9407: F, t3064: F, t6240: F, t2196: F, t9129: F, t7313: F, t7551: F, t2651: F, t7597: F, t2133: F, t3115: F, t6848: F) -> (F, F, F, F, F, F, F) {
    let t29517 = t1567 * t3177;
    let t29524 = t2184 * t551 * t1632 * t9407;
    let t29533 = t6240 * t3064;
    let t29544 = t2196 * t551 * t1632 * t9129;
    let t29552 = t7313 * t7551;
    let t29561 = t2651 * t7597;
    let t29568 = t2133 * t6848 * t3115;
    (t29517, t29524, t29533, t29544, t29552, t29561, t29568)
}
