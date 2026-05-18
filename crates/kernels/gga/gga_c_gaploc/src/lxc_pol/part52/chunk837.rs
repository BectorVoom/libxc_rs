//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 837/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk837<F: Float>(t2508: F, t2541: F, t36654: F, t11931: F, t2558: F, t943: F, t13552: F, t2549: F, t11595: F, t7667: F, t35682: F, t7659: F) -> (F, F, F, F, F) {
    let t44812 = F::new(0.11535789345213336425e0) * t2508 * t2541 * t36654;
    let t44817 = t943 * t11931 * t2558;
    let t44818 = F::new(0.32043859292259267849e-3) * t44817;
    let t44819 = t2549 * t13552;
    let t44820 = F::new(0.32043859292259267849e-3) * t44819;
    let t44823 = F::new(0.53833683610995569986e-1) * t2508 * t11595 * t7667;
    let t44826 = F::new(0.92286314761706691403e-1) * t2508 * t35682 * t7659;
    (t44812, t44818, t44820, t44823, t44826)
}
