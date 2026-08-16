//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 602/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk602<F: Float>(t1789: F, t372: F, t371: F, t1721: F, t1735: F, t1761: F, t1763: F, t1767: F) -> (F, F) {
    let t1790 = t372 * t1789;
    let t1791 = t371 * t1790;
    let t1794 = -t1721 + t1735 + t1761 + t1763 - t1767;
    (t1791, t1794)
}
