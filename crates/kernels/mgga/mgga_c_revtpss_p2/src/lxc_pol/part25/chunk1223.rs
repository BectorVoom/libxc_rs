//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1223/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1223<F: Float>(t2430: F, t605: F, t2257: F, t775: F, t2394: F, t11054: F, t30: F, t10489: F, t198: F, t206: F, t7086: F, t10627: F) -> (F, F, F, F, F, F, F) {
    let t92795 = t605 * t2430;
    let t92799 = t2257 * t775;
    let t92806 = t605 * t2394;
    let t92810 = t30 * t11054;
    let t92814 = t30 * t10489;
    let t92819 = t198 * t206 * t7086;
    let t92822 = t198 * t10627;
    (t92795, t92799, t92806, t92810, t92814, t92819, t92822)
}
