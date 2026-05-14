//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 800/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk800<F: Float>(t4534: F, t4582: F, t1123: F, t198: F, t1129: F, t1539: F, t1160: F, t1297: F, t1535: F, t502: F, t535: F, t17: F, t2849: F, t1996: F, t550: F, t19: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4583 = t4534 + t4582;
    let t4711 = t198 * t1123;
    let t4714 = t198 * t1129;
    let t4851 = t198 * t1539;
    let t4861 = t1160 * t1297;
    let t5198 = t502 * t1535;
    let t5204 = t535 * t1535;
    let t5471 = t17 * t2849;
    let t5861 = t550 * t1996;
    let t5862 = t19 * t5861;
    (t4583, t4711, t4714, t4851, t4861, t5198, t5204, t5471, t5861, t5862)
}
