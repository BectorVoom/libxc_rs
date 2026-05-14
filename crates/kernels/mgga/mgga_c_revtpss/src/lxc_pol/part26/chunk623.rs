//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 623/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk623<F: Float>(t640: F, t76: F, t112: F, t624: F, t655: F, t68: F, t665: F, t30: F, t775: F, t159: F, t793: F) -> (F, F, F, F, F, F) {
    let t6977 = t76 * t640;
    let t6996 = t624 * t112;
    let t6998 = t68 * t655;
    let t6999 = t6998 * t665;
    let t7010 = t30 * t775;
    let t7021 = t793 * t159;
    (t6977, t6996, t6998, t6999, t7010, t7021)
}
