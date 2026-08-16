//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2475/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2475<F: Float>(t1068: F, t3209: F, t13666: F, t14667: F, t4700: F, t49228: F, t49544: F, t49548: F, t49550: F, t49552: F, t49556: F, t49558: F, t49560: F, t49562: F) -> F {
    let t50775 = t3209 * t1068;
    let t50779 = -F::cast_from(3.0_f64) * t13666 * t3209 * t4700 + F::cast_from(6.0_f64) * t14667 * t4700 * t50775 + t49228 - t49544 + t49548 - t49550 - t49552 - t49556 - t49558 + t49560 - t49562;
    t50779
}
