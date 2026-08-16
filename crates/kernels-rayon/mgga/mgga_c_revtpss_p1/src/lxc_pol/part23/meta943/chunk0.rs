//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3096/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3096(t1732: f64, t3433: f64, t69591: f64, t20644: f64, t5104: f64, t5068: f64, t68792: f64, t5109: f64, t68952: f64, t17092: f64, t20641: f64, t16840: f64, t20645: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81618 = 0.48245938496077605201e2_f64 * t3433 * t69591 * t1732;
    let t81621 = 0.48245938496077605201e2_f64 * t3433 * t20644 * t5104;
    let t81623 = 6.0_f64 * t68792 * t5068;
    let t81625 = 0.48245938496077605201e2_f64 * t68952 * t5109;
    let t81627 = 6.0_f64 * t17092 * t20641;
    let t81629 = 0.48245938496077605201e2_f64 * t16840 * t20645;
    (t81618, t81621, t81623, t81625, t81627, t81629)
}
