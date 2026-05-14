//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 365/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk365<F: Float>(t1785: F, t480: F, t1774: F, t482: F, t372: F, t371: F, t1721: F, t1735: F, t1761: F, t1763: F, t1767: F) -> (F, F, F, F) {
    let t1786 = t1785 * t480;
    let t1789 = t482 * t1774;
    let t1790 = t372 * t1789;
    let t1791 = t371 * t1790;
    let t1794 = -t1721 + t1735 + t1761 + t1763 - t1767;
    (t1786, t1789, t1791, t1794)
}
