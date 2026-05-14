//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 183/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk183<F: Float>(t787: F, t789: F, t579: F, t65: F, t64: F, t159: F, t222: F, t228: F, t216: F, t136: F, t220: F) -> (F, F, F, F, F, F, F, F) {
    let t791 = 0.9757440539382783019e-2 * t787 * t789;
    let t793 = 1.0 / t65 / t579;
    let t794 = t64 * t793;
    let t795 = t794 * t159;
    let t797 = 7.0 / 288.0 * t795 * t222;
    let t798 = t159 * t228;
    let t799 = t216 * t798;
    let t800 = t136 * t220;
    (t791, t793, t794, t795, t797, t798, t799, t800)
}
