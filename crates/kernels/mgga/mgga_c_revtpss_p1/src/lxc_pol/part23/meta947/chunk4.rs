//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3132/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3132<F: Float>(t81582: F, t81589: F, t81591: F, t81593: F, t81596: F, t81599: F, t81601: F, t81604: F, t81606: F, t81609: F, t81612: F, t81614: F, t81618: F, t81621: F, t81623: F, t81625: F, t81627: F, t81629: F, t81631: F, t81633: F) -> F {
    let t82388 = -t81582 + t81589 + t81591 - t81593 - t81596 + t81599 - t81601 - t81604 - t81606 + t81609 - t81612 + t81614 + t81618 + t81621 - t81623 + t81625 - t81627 + t81629 - t81631 + t81633;
    t82388
}
