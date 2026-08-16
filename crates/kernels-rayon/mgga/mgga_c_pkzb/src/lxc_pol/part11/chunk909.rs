//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 909/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk909(t2240: f64, t9847: f64, t1185: f64, t3069: f64, t2197: f64, t3769: f64, t851: f64, t6142: f64, t3766: f64, t2242: f64, t3765: f64, t9766: f64, t9768: f64, t9770: f64, t9840: f64, t9842: f64, t9844: f64, t9846: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9849 = 6.0_f64 * t2240 * t9847;
    let t9850 = t1185 * t3069;
    let t9852 = 4.0_f64 * t2197 * t9850;
    let t9853 = t3769 * t851;
    let t9855 = 0.96491876992155210402e2_f64 * t6142 * t9853;
    let t9856 = t3766 * t851;
    let t9858 = 2.0_f64 * t2197 * t9856;
    let t9859 = t3765 * t2242;
    let t9860 = t9859 * t851;
    let t9862 = 0.16081979498692535067e2_f64 * t2240 * t9860;
    let t9863 = -t9766 + t9768 - t9770 - t9840 - t9842 + t9844 - t9846 - t9849 + t9852 + t9855 + t9858 - t9862;
    (t9849, t9850, t9852, t9853, t9855, t9856, t9858, t9859, t9860, t9862, t9863)
}
