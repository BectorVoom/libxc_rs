//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1102/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1102<F: Float>(t833: F, t9838: F, t3769: F, t6137: F, t3038: F, t8009: F, t3074: F, t8219: F, t3740: F, t851: F, t2240: F, t1185: F, t3069: F, t2197: F, t6142: F, t3766: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9840 = 1.0 * t833 * t9838;
    let t9842 = 0.16081979498692535067e2 * t6137 * t3769;
    let t9844 = 4.0 * t8009 * t3038;
    let t9846 = 0.32163958997385070134e2 * t8219 * t3074;
    let t9847 = t3740 * t851;
    let t9849 = 6.0 * t2240 * t9847;
    let t9850 = t1185 * t3069;
    let t9852 = 4.0 * t2197 * t9850;
    let t9853 = t3769 * t851;
    let t9855 = 0.96491876992155210402e2 * t6142 * t9853;
    let t9856 = t3766 * t851;
    (t9840, t9842, t9844, t9846, t9847, t9849, t9850, t9852, t9853, t9855, t9856)
}
