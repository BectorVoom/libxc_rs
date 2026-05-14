//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1312/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1312<F: Float>(t2927: F, t30657: F, t11377: F, t1828: F, t11406: F, t11376: F, t11410: F, t30570: F, t1161: F, t2876: F, t4512: F, t11266: F, t7785: F, t11282: F, t11315: F, t22531: F, t2829: F, t2869: F, t30571: F, t3739: F, t3747: F, t7637: F, t7643: F, t7800: F, t7806: F, t9587: F, t9594: F, t9657: F) -> (F, F, F, F, F, F, F) {
    let t30682 = t2927 * t30657;
    let t30685 = t11377 * t1828;
    let t30686 = t11406 * t30685;
    let t30689 = t11376 * t30685;
    let t30692 = t11410 * t30685;
    let t30697 = t11406 * t30570;
    let t30703 = t1161 * t4512 * t2876;
    let t30710 = t11266 * t7785;
    let t30716 = t11282 * t7785;
    let t30719 = 512.0 / 81.0 * t30682 * t9657 + 5632.0 / 2187.0 * t9587 * t30686 + 704.0 / 81.0 * t3747 * t30689 + 1408.0 / 243.0 * t3739 * t30692 + 5632.0 / 2187.0 * t9594 * t30686 + 128.0 / 3.0 * t7806 * t30697 + 256.0 / 81.0 * t22531 * t30571 + 616.0 / 9.0 * t7637 * t30703 + 440.0 / 9.0 * t7800 * t1161 * t4512 * t2869 + 88.0 / 9.0 * t7643 * t30710 + 440.0 / 9.0 * t7800 * t11315 * t7785 - 88.0 / 27.0 * t2829 * t30716;
    (t30689, t30692, t30697, t30703, t30710, t30716, t30719)
}
