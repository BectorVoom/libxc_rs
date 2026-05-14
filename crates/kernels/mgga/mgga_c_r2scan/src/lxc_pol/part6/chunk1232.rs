//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1232/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1232<F: Float>(t1543: F, t2196: F, t551: F, t6343: F, t146: F, t20200: F, t548: F, t1632: F, t5054: F, t6528: F, t5052: F, t524: F, t525: F, t1541: F, t1598: F, t6524: F) -> (F, F, F, F, F, F) {
    let t22829 = t2196 * t551 * t6343 * t1543;
    let t22836 = t146 * t20200 * t548;
    let t22843 = t6528 * t551 * t1632 * t5054;
    let t22850 = t524 * t525 * t5052;
    let t22856 = t524 * t1598 * t1541;
    let t22857 = t22856 * t6524;
    (t22829, t22836, t22843, t22850, t22856, t22857)
}
