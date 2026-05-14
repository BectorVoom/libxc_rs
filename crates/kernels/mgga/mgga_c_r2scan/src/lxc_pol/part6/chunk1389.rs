//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1389/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1389<F: Float>(t2133: F, t2294: F, t7429: F, t4715: F, t5: F, t966: F, t1398: F, t2804: F, t378: F, t7854: F, t21056: F, t21060: F, t595: F, t898: F, t22418: F, t21048: F, t21050: F, t21052: F, t21054: F, t21065: F, t21069: F, t21088: F, t21091: F, t21094: F, t21097: F) -> (F, F, F, F, F) {
    let t26340 = t2133 * t2294 * t7429;
    let t26356 = t5 * t4715 * t966;
    let t26359 = t5 * t1398 * t2804;
    let t26360 = 20.0 / 3.0 * t26359;
    let t26362 = t5 * t378 * t7854;
    let t26367 = 96.0 * t21056;
    let t26368 = 960.0 * t21060;
    let t26369 = t595 * t898;
    let t26370 = t26369 * t22418;
    let t26372 = t21048 - 0.54217906501508699211e-2 * t21050 - 0.32530743900905219526e-1 * t21052 - 0.48796115851357829289e-1 * t21054 + t26367 + t26368 - t21065 + t21069 - t21088 - t21091 - 0.10005107252466666666e-1 * t26370 - t21094 + t21097;
    (t26340, t26356, t26360, t26362, t26372)
}
