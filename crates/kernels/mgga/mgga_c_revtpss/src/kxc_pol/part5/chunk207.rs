//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 207/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk207<F: Float>(t114: F, t100: F, t658: F, t108: F, t101: F, t105: F, t656: F, t97: F, t655: F, t653: F, t69: F) -> (F, F, F, F, F) {
    let t115 = 1.0 < t114;
    let t659 = t100 * t658;
    let t661 = -t658;
    let t662 = t108 * t661;
    let t665 = -5.0 / 3.0 * t656 * t101 + 5.0 / 3.0 * t105 * t662 + 5.0 / 3.0 * t97 * t659;
    let t666 = t655 * t665;
    let t670 = piecewise3(t115, 0.0, -t653 - t69 * t666 / 8.0);
    (t661, t662, t665, t666, t670)
}
