//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2588/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2588<F: Float>(t51399: F, t51401: F, t51404: F, t51437: F, t51439: F, t51441: F, t51443: F, t51446: F, t51449: F, t51453: F, t51456: F, t51459: F, t51463: F, t51466: F, t51806: F, t51809: F, t51814: F, t51818: F, t51822: F, t51824: F) -> F {
    let t52451 = -t51806 - t51809 + t51399 + t51401 + t51404 - t51814 + t51818 - t51822 + t51824 - t51437 - t51439 + t51441 + t51443 - t51446 - t51449 - t51453 - t51456 + t51459 + t51463 + t51466;
    t52451
}
