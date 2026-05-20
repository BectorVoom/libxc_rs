//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3098/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3098<F: Float>(t81618: F, t81621: F, t81623: F, t81625: F, t81627: F, t81629: F, t81631: F, t81633: F, t81635: F, t81638: F, t81641: F, t1149: F, t24327: F, t44017: F) -> (F, F) {
    let t81642 = t81618 + t81621 - t81623 + t81625 - t81627 + t81629 - t81631 + t81633 + t81635 + t81638 - t81641;
    let t81646 = F::cast_from(0.62071215503128080361e4_f64) * t44017 * t24327 * t1149;
    (t81642, t81646)
}
