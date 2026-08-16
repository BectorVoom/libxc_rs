//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1301/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1301(t1149: f64, t6471: f64, t3384: f64, t3435: f64, t6470: f64, t3433: f64, t5104: f64, t5108: f64, t12230: f64, t6438: f64, t12227: f64, t1187: f64, t6519: f64) -> (f64, f64, f64, f64, f64) {
    let t20641 = t6471 * t1149;
    let t20643 = 2.0_f64 * t3384 * t20641;
    let t20644 = t6470 * t3435;
    let t20645 = t20644 * t1149;
    let t20647 = 0.16081979498692535067e2_f64 * t3433 * t20645;
    let t20648 = t5108 * t5104;
    let t20650 = 0.32163958997385070134e2_f64 * t3433 * t20648;
    let t20651 = t6438 * t12230;
    let t20652 = t20651 * t1149;
    let t20654 = 0.51726012919273400301e3_f64 * t12227 * t20652;
    let t20659 = t6519 * t1187;
    (t20643, t20647, t20650, t20654, t20659)
}
