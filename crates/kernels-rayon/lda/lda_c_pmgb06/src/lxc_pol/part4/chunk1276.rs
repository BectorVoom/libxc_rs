//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1276/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1276(t12982: f64, t13008: f64, t4937: f64, t831: f64, t13079: f64, t9890: f64, t9892: f64, t9895: f64, t9898: f64, t16755: f64, t16757: f64, t16759: f64, t16773: f64, t16775: f64, t16780: f64, t16781: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16782 = 8.0_f64 / 81.0_f64 * t12982;
    let t16783 = 16.0_f64 / 135.0_f64 * t13008;
    let t16785 = t831 * t4937 / 15.0_f64;
    let t16786 = 8.0_f64 / 135.0_f64 * t13079;
    let t16787 = 8.0_f64 / 405.0_f64 * t9890;
    let t16788 = 2.0_f64 / 135.0_f64 * t9892;
    let t16789 = 2.0_f64 / 135.0_f64 * t9895;
    let t16790 = 2.0_f64 / 135.0_f64 * t9898;
    let t16791 = t16755 + t16757 + t16759 + t16773 + t16775 + t16780 + t16781 - t16782 + t16783 - t16785 + t16786 - t16787 + t16788 + t16789 - t16790;
    (t16782, t16783, t16785, t16786, t16787, t16788, t16789, t16790, t16791)
}
