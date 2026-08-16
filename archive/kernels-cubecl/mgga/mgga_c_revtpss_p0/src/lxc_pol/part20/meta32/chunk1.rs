//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 240/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk240<F: Float>(t114: F, t100: F, t658: F, t108: F, t101: F, t105: F, t656: F, t97: F, t655: F, t653: F, t69: F) -> (F, F, F, F, F) {
    let t115 = F::cast_from(1.0_f64) < t114;
    let t659 = t100 * t658;
    let t661 = -t658;
    let t662 = t108 * t661;
    let t665 = -F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t656 * t101 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t105 * t662 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t97 * t659;
    let t666 = t655 * t665;
    let t670 = piecewise3::<F>(t115, F::cast_from(0.0_f64), -t653 - t69 * t666 / F::cast_from(8.0_f64));
    (t659, t661, t665, t666, t670)
}
