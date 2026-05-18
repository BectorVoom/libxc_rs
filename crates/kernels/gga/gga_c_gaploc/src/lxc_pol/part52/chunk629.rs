//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 629/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk629<F: Float>(t1: F, t11679: F, t787: F, t11684: F, t549: F, t10877: F, t10880: F, t10884: F, t10887: F, t10890: F, t10898: F, t10900: F, t10920: F, t10943: F, t6111: F, t6119: F, t9982: F) -> (F, F, F) {
    let t11822 = t11679 * t1;
    let t11823 = t787 * t11822;
    let t11826 = t549 * t11684;
    let t11829 = F::new(0.76685851907841499353e0) * t10877 + F::new(0.76685851907841499353e0) * t10880 - F::new(0.17041300423964777634e0) * t10884 + F::new(0.59584149919750711116e-1) * t10887 - F::new(0.59584149919750711116e-1) * t10890 + F::new(0.17041300423964777634e0) * t10898 + F::new(0.59584149919750711116e-1) * t10900 + F::new(0.11916829983950142223e0) * t10920 + F::new(0.12780975317973583225e0) * t9982 + F::new(0.38342925953920749677e1) * t10943 + F::new(0.11916829983950142223e0) * t11823 * t6119 - F::new(0.79445533226334281487e-1) * t6111 * t11826;
    (t11822, t11823, t11829)
}
