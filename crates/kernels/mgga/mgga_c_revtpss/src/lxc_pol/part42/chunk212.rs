//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 212/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk212<F: Float>(t108: F, t661: F, t101: F, t105: F, t656: F, t659: F, t97: F) -> (F, F) {
    let t662 = t108 * t661;
    let t665 = -5.0 / 3.0 * t656 * t101 + 5.0 / 3.0 * t105 * t662 + 5.0 / 3.0 * t97 * t659;
    (t662, t665)
}
