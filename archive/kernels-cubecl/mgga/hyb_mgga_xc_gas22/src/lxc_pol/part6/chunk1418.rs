//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1418/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1418<F: Float>(t2927: F, t30657: F, t11377: F, t1828: F, t11406: F, t11376: F, t11410: F, t30570: F, t1161: F, t2876: F, t4512: F, t11266: F, t7785: F) -> (F, F, F, F, F, F, F) {
    let t30682 = t2927 * t30657;
    let t30685 = t11377 * t1828;
    let t30686 = t11406 * t30685;
    let t30689 = t11376 * t30685;
    let t30692 = t11410 * t30685;
    let t30697 = t11406 * t30570;
    let t30703 = t1161 * t4512 * t2876;
    let t30710 = t11266 * t7785;
    (t30682, t30686, t30689, t30692, t30697, t30703, t30710)
}
