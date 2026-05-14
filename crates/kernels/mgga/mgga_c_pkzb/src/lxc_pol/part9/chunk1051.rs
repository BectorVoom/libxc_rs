//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1051/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1051<F: Float>(t1816: F, t637: F, t46: F, t552: F, t6798: F, t1548: F, t2607: F, t16632: F, t135: F, t1634: F, t1009: F, t4882: F, t5137: F, t16638: F, t1508: F, t7035: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19790 = t637 * t1816;
    let t19795 = t6798 * t46 * t552;
    let t19796 = 0.54934341918019635162e-3 * t19795;
    let t19797 = t1548 * t2607;
    let t19798 = 96.0 * t19797;
    let t19799 = 36.0 * t16632;
    let t19800 = t135 * t1634;
    let t19803 = t4882 * t1009;
    let t19804 = 240.0 * t19803;
    let t19805 = t5137 * t1009;
    let t19806 = 120.0 * t19805;
    let t19807 = 180.0 * t16638;
    let t19809 = t1634 * t637;
    let t19822 = t7035 * t1508;
    (t19790, t19796, t19798, t19799, t19800, t19804, t19806, t19807, t19809, t19822)
}
