//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 735/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk735<F: Float>(t44819: F, t11595: F, t2508: F, t7667: F, t35682: F, t7659: F, t13552: F, t731: F, t13503: F, t2549: F, t40612: F, t40614: F, t40620: F, t40622: F, t40627: F, t40630: F, t40632: F, t40634: F, t471: F) -> (F, F, F, F, F, F, F) {
    let t44820 = 0.32043859292259267849e-3 * t44819;
    let t44823 = 0.53833683610995569986e-1 * t2508 * t11595 * t7667;
    let t44826 = 0.92286314761706691403e-1 * t2508 * t35682 * t7659;
    let t44827 = t731 * t13552;
    let t44828 = 0.42725145723012357132e-3 * t44827;
    let t44829 = t731 * t13503;
    let t44837 = t2549 * t13503;
    let t44855 = (21.0 / 256.0 * t40612 + 357.0 / 8192.0 * t40614 - 189.0 / 131072.0 * t40620 + 189.0 / 8388608.0 * t40622 - 63.0 / 8388608.0 * t40627 + 63.0 / 131072.0 * t40630 - 119.0 / 8192.0 * t40632 - 7.0 / 256.0 * t40634) * t471;
    (t44820, t44823, t44826, t44828, t44829, t44837, t44855)
}
