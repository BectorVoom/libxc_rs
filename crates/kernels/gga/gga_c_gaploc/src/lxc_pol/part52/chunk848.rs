//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 848/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk848<F: Float>(t12272: F, t14350: F, t44671: F, t44674: F, t44676: F, t44678: F, t44684: F, t44687: F, t44689: F, t44692: F, t44694: F, t44697: F, t44702: F, t44705: F, t49820: F, t49965: F, t49968: F, t49998: F, t50029: F, t50074: F, t50108: F, t50134: F, t50136: F, t50149: F, t50163: F, t50179: F, t50208: F, t50239: F, t50253: F, t50263: F, t50276: F, t50286: F, t50302: F, t5552: F, t748: F, t8862: F) -> (F,) {
    let t50308 = -t44697 + t44671 + 4.0 * t5552 * t14350 + t44674 + t49820 + 4.0 * t8862 * t12272 - t44676 + t44678 + t44684 - t44687 + t44689 - t44692 + t44694 - t44702 + t44705 - t748 * (t49998 + t50029 + t50074 + t50108 + t50134 + t50136 + t50149 + t50163 + t50179 + t50208 + t50239 + t50253 + t50263 + t50276 + t50286 + t50302) - t49965 - t49968;
    (t50308,)
}
