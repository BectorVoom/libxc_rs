//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1395/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1395<F: Float>(t1014: F, t21601: F, t29650: F, t29652: F, t29654: F, t29656: F, t29658: F, t29660: F, t29663: F, t29666: F, t29669: F, t29671: F, t29674: F, t3591: F, t4310: F, t9001: F, t9002: F) -> F {
    let t30235 = t29650 + t29652 - t29654 - t29656 + t29658 + t29660 + t29663 + t29666 - t29669 - t29671 + t29674 + F::new(0.12304822629859687989e5) * t1014 * t21601 * t4310 * t9001 - F::new(0.20508037716432813315e4) * t3591 * t9002;
    t30235
}
