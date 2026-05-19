//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 909/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk909<F: Float>(t2240: F, t9847: F, t1185: F, t3069: F, t2197: F, t3769: F, t851: F, t6142: F, t3766: F, t2242: F, t3765: F, t9766: F, t9768: F, t9770: F, t9840: F, t9842: F, t9844: F, t9846: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9849 = F::new(6.0) * t2240 * t9847;
    let t9850 = t1185 * t3069;
    let t9852 = F::new(4.0) * t2197 * t9850;
    let t9853 = t3769 * t851;
    let t9855 = F::cast_from(0.96491876992155210402e2_f64) * t6142 * t9853;
    let t9856 = t3766 * t851;
    let t9858 = F::new(2.0) * t2197 * t9856;
    let t9859 = t3765 * t2242;
    let t9860 = t9859 * t851;
    let t9862 = F::cast_from(0.16081979498692535067e2_f64) * t2240 * t9860;
    let t9863 = -t9766 + t9768 - t9770 - t9840 - t9842 + t9844 - t9846 - t9849 + t9852 + t9855 + t9858 - t9862;
    (t9849, t9850, t9852, t9853, t9855, t9856, t9858, t9859, t9860, t9862, t9863)
}
