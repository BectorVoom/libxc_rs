//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1225/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1225<F: Float>(t26359: F, t21056: F, t21060: F, t595: F, t898: F, t22418: F, t1726: F, t2798: F, t5228: F, t5943: F, t7666: F, t2788: F, t5893: F, t1759: F, t584: F, t7778: F) -> (F, F, F, F, F, F, F, F) {
    let t26360 = 20.0 / 3.0 * t26359;
    let t26367 = 96.0 * t21056;
    let t26368 = 960.0 * t21060;
    let t26369 = t595 * t898;
    let t26370 = t26369 * t22418;
    let t26374 = t1726 * t2798 * t5228;
    let t26376 = t7666 * t5943;
    let t26378 = t2788 * t5893;
    let t26381 = t584 * t7778 * t1759;
    (t26360, t26367, t26368, t26370, t26374, t26376, t26378, t26381)
}
