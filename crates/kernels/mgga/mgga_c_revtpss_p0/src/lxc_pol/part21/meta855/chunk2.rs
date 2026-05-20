//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3237/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3237<F: Float>(t1300: F, t198: F, t336: F, t56390: F, t56484: F, t56534: F, t56593: F, t56642: F, t56687: F, t57794: F, t57799: F, t57802: F, t57805: F, t57808: F, t57810: F, t57812: F, t57814: F, t57816: F, t57820: F, t60068: F, t60117: F) -> F {
    let t60124 = t198 * t336 * (t56390 + t56484 + t56534 + t56593 + t56642 + t56687 + t60068 + t60117) * t1300 - t57794 + t57799 - t57802 - t57805 - t57808 - t57810 - t57812 - t57814 + t57816 - t57820;
    t60124
}
