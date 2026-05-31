//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1345/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1345<F: Float>(t1149: F, t6471: F, t3384: F, t3435: F, t6470: F, t3433: F, t5104: F, t5108: F, t12230: F, t6438: F, t12227: F, t1187: F, t6519: F) -> (F, F, F, F, F) {
    let t20641 = t6471 * t1149;
    let t20643 = F::cast_from(2.0_f64) * t3384 * t20641;
    let t20644 = t6470 * t3435;
    let t20645 = t20644 * t1149;
    let t20647 = F::cast_from(0.16081979498692535067e2_f64) * t3433 * t20645;
    let t20648 = t5108 * t5104;
    let t20650 = F::cast_from(0.32163958997385070134e2_f64) * t3433 * t20648;
    let t20651 = t6438 * t12230;
    let t20652 = t20651 * t1149;
    let t20654 = F::cast_from(0.51726012919273400301e3_f64) * t12227 * t20652;
    let t20659 = t6519 * t1187;
    (t20643, t20647, t20650, t20654, t20659)
}
