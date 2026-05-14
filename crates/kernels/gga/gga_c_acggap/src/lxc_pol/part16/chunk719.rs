//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 719/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk719<F: Float>(t1429: F, t2001: F, t1418: F, t7383: F, t7387: F, t7390: F, t7396: F, t7405: F, t8680: F, t8682: F, t8684: F, t8686: F, t8690: F, t8692: F, t8694: F, t8696: F) -> (F,) {
    let t8698 = t2001 * t1429;
    let t8700 = t2001 * t1418;
    let t8702 = -t7383 / 64.0 - t7387 / 192.0 - 0.7640625e-2 * t7390 + 0.140078125e-1 * t7396 + 7.0 / 288.0 * t7405 + 11.0 / 384.0 * t8680 + 11.0 / 1152.0 * t8682 + 7.0 / 144.0 * t8684 + 0.25724410870841842183e-2 * t8686 - 0.10718504529517434243e-3 * t8690 - 0.17149607247227894789e-2 * t8692 + 0.85748036236139473944e-3 * t8694 + 0.34299214494455789578e-2 * t8696 + 0.85748036236139473945e-2 * t8698 - 0.34299214494455789578e-2 * t8700;
    (t8702,)
}
