//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1042/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1042<F: Float>(t169: F, t3994: F, t3975: F, t6507: F, t2109: F, t711: F, t151: F, t715: F, t1852: F, t3981: F, t3979: F, t6528: F, t674: F, t8645: F, t6536: F, t3188: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10674 = t169 * t3994;
    let t10677 = t6507 * t3975;
    let t10682 = t2109 * t3994;
    let t10685 = t711 * t3975;
    let t10690 = t151 * t3994;
    let t10693 = t715 * t3975;
    let t10699 = t1852 * t3981;
    let t10701 = t6528 * t3979;
    let t10703 = t8645 * t10701 * t674;
    let t10706 = t6536 * t3979;
    let t10708 = t3188 * t10706 * t674;
    (t10674, t10677, t10682, t10685, t10690, t10693, t10699, t10701, t10703, t10706, t10708)
}
