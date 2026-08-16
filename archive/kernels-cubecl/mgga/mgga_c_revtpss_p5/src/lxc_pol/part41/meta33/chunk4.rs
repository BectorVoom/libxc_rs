//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 211/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk211<F: Float>(t100: F, t658: F, t108: F, t101: F, t105: F, t656: F, t97: F) -> (F, F, F) {
    let t659 = t100 * t658;
    let t661 = -t658;
    let t662 = t108 * t661;
    let t665 = -F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t656 * t101 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t105 * t662 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t97 * t659;
    (t661, t662, t665)
}
