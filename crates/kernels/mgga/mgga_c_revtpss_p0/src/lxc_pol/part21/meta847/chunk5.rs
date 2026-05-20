//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3180/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3180<F: Float>(t1131: F, t1150: F, t58491: F, t58504: F, t58518: F, t58531: F, t58545: F, t58558: F, t58572: F, t58585: F, t12470: F, t1744: F) -> (F, F) {
    let t58591 = F::new(1.0) * t1131 * (t58491 + t58504 + t58518 + t58531 + t58545 + t58558 + t58572 + t58585) * t1150;
    let t58592 = t12470 * t1744;
    (t58591, t58592)
}
