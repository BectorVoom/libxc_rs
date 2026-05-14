//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1361/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1361<F: Float>(t25766: F, t25767: F, t6064: F, t2177: F, t7301: F, t1592: F, t2654: F, t551: F, t6343: F, t22856: F, t8191: F, t1632: F, t6449: F, t7576: F, t1610: F, t2207: F, t8270: F) -> (F, F, F, F, F, F) {
    let t25769 = t25766 * t25767 * t6064;
    let t25773 = t2177 * t7301;
    let t25779 = t1592 * t551 * t6343 * t2654;
    let t25780 = 0.38140175656238781678e1 * t25779;
    let t25781 = t22856 * t8191;
    let t25793 = t6449 * t551 * t1632 * t7576;
    let t25797 = t2207 * t1610 * t8270;
    (t25769, t25773, t25780, t25781, t25793, t25797)
}
