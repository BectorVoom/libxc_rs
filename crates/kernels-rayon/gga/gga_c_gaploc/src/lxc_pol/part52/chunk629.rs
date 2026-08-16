//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 629/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk629(t1: f64, t11679: f64, t787: f64, t11684: f64, t549: f64, t10877: f64, t10880: f64, t10884: f64, t10887: f64, t10890: f64, t10898: f64, t10900: f64, t10920: f64, t10943: f64, t6111: f64, t6119: f64, t9982: f64) -> (f64, f64, f64) {
    let t11822 = t11679 * t1;
    let t11823 = t787 * t11822;
    let t11826 = t549 * t11684;
    let t11829 = 0.76685851907841499353e0_f64 * t10877 + 0.76685851907841499353e0_f64 * t10880 - 0.17041300423964777634e0_f64 * t10884 + 0.59584149919750711116e-1_f64 * t10887 - 0.59584149919750711116e-1_f64 * t10890 + 0.17041300423964777634e0_f64 * t10898 + 0.59584149919750711116e-1_f64 * t10900 + 0.11916829983950142223e0_f64 * t10920 + 0.12780975317973583225e0_f64 * t9982 + 0.38342925953920749677e1_f64 * t10943 + 0.11916829983950142223e0_f64 * t11823 * t6119 - 0.79445533226334281487e-1_f64 * t6111 * t11826;
    (t11822, t11823, t11829)
}
