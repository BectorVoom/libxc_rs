//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2267/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2267<F: Float>(t17092: F, t25200: F, t2718: F, t4147: F, t4300: F, t6663: F, t7537: F, t82209: F, t82211: F, t82219: F, t855: F, t87805: F, t98927: F, t98932: F, t98941: F, t98945: F) -> F {
    let t98947 = F::cast_from(0.82246703342411321825e-2_f64) * t98927 - F::cast_from(2.0_f64) * t17092 * t6663 - t87805 - F::cast_from(0.12793931631041761173e0_f64) * t82209 + F::cast_from(0.38381794893125283518e-1_f64) * t98932 - F::cast_from(0.63969658155208805863e-1_f64) * t82211 + F::cast_from(4.0_f64) * t855 * t2718 * t7537 * t4300 + F::cast_from(4.0_f64) * t4147 * t25200 - t82219 - F::cast_from(0.76763589786250567037e-1_f64) * t98941 - F::cast_from(0.82246703342411321825e-2_f64) * t98945;
    t98947
}
