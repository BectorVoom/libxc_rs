//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 686/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk686<F: Float>(t1676: F, t637: F, t13: F, t25: F, t1410: F, t452: F, t1448: F, t30: F, t1450: F, t448: F, t1444: F, t459: F, t1466: F, t292: F, t4: F, t14: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4025 = t1676 * t637;
    let t4494 = t13 * t13;
    let t4635 = t25 * t25;
    let t4769 = t1410 * t452;
    let t4772 = t30 * t1448;
    let t4773 = t448 * t1450;
    let t4776 = t1444 * t459;
    let t4779 = t448 * t1466;
    let t4783 = 1.0 / t4 / t292;
    let t4784 = sigma0 * t4783;
    let t4793 = t14 * t13;
    (t4025, t4494, t4635, t4769, t4772, t4773, t4776, t4779, t4784, t4793)
}
