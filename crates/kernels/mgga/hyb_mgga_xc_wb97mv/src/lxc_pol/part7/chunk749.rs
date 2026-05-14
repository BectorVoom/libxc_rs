//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 749/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk749<F: Float>(t154: F, t3975: F, t3994: F, t711: F, t157: F, t715: F, t160: F, t719: F, t163: F, t723: F, t166: F, t727: F, t169: F, t731: F, t2109: F, t735: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3997 = t154 * t3975;
    let t3999 = t711 * t3994;
    let t4001 = t157 * t3975;
    let t4003 = t715 * t3994;
    let t4005 = t160 * t3975;
    let t4007 = t719 * t3994;
    let t4009 = t163 * t3975;
    let t4011 = t723 * t3994;
    let t4013 = t166 * t3975;
    let t4015 = t727 * t3994;
    let t4017 = t169 * t3975;
    let t4019 = t731 * t3994;
    let t4021 = t2109 * t3975;
    let t4023 = t735 * t3994;
    (t3997, t3999, t4001, t4003, t4005, t4007, t4009, t4011, t4013, t4015, t4017, t4019, t4021, t4023)
}
