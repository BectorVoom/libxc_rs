//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 217/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk217(t265: f64, t260: f64, t350: f64, t405: f64, t624: f64, t629: f64) -> (f64, f64, f64, f64) {
    let t659 = t265 * t265;
    let t660 = 1.0_f64 / t659;
    let t661 = t260 * t660;
    let t666 = -1.176575_f64 * t624 - 0.516475_f64 * t350 - 0.2103875_f64 * t629 - 0.104195_f64 * t405;
    (t659, t660, t661, t666)
}
