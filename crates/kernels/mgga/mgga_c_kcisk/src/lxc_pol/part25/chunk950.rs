//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 950/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk950<F: Float>(t1648: F, t2487: F, t1824: F, t16804: F, t3521: F, t7047: F, t6790: F, t695: F, t1060: F, t4609: F, t3293: F, t7045: F, t4597: F, t3290: F, t11285: F, t682: F, t7028: F) -> (F, F, F, F, F, F) {
    let t16805 = t2487 * t1648;
    let t16806 = t16805 * t1824;
    let t16807 = t16804 * t16806;
    let t16810 = t3521 * t7047;
    let t16812 = t6790 * t695;
    let t16813 = t16812 * t1060;
    let t16814 = t4609 * t16813;
    let t16817 = t7045 * t3293;
    let t16818 = t4609 * t16817;
    let t16821 = t2487 * t4597;
    let t16822 = t16821 * t3290;
    let t16823 = t11285 * t16822;
    let t16826 = t7028 * t682;
    (t16807, t16810, t16814, t16818, t16823, t16826)
}
