//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1101/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1101<F: Float>(t20868: F, t20888: F, t664: F, t684: F, t1854: F, t2743: F, t1857: F, t1070: F, t5801: F, t5805: F, t1084: F, t17650: F, t2783: F, t5766: F, t1850: F, t7444: F) -> (F, F, F, F, F, F) {
    let t20892 = 1.0 * t664 * (t20868 + t20888) * t684;
    let t20893 = t2743 * t1854;
    let t20895 = 6.0 * t20893 * t1857;
    let t20896 = t1070 * t5801;
    let t20898 = 0.51726012919273400301e3 * t20896 * t5805;
    let t20900 = 1.0 * t17650 * t1084;
    let t20902 = 3.0 * t5766 * t2783;
    let t20904 = 3.0 * t1850 * t7444;
    (t20892, t20895, t20898, t20900, t20902, t20904)
}
