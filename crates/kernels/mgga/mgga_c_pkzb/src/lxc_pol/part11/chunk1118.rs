//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1118/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1118<F: Float>(t3532: F, t7370: F, t2765: F, t9164: F, t10806: F, t1873: F, t667: F, t10800: F, t17432: F, t2759: F, t9137: F, t7365: F, t2754: F, t1861: F, t1066: F, t218: F, t219: F, t9161: F) -> (F, F, F, F, F, F, F, F, F) {
    let t30314 = t7370 * t3532;
    let t30316 = t2765 * t9164;
    let t30319 = t1873 * t10806 * t667;
    let t30322 = t17432 * t10800 * t667;
    let t30324 = t9137 * t2759;
    let t30326 = t7365 * t3532;
    let t30328 = t2754 * t9164;
    let t30331 = t1861 * t10806 * t667;
    let t30338 = t218 * t219 * t1066 * t9161;
    (t30314, t30316, t30319, t30322, t30324, t30326, t30328, t30331, t30338)
}
