//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1189/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1189(t2448: f64, t27: f64, t402: f64, t5770: f64, t5783: f64, t110: f64, t360: f64, t7313: f64, t18755: f64, t18757: f64, t18759: f64, t2209: f64, t2233: f64, t2712: f64, t342: f64, t35: f64, t5980: f64, t7278: f64, t7310: f64, t780: f64) -> (f64, f64) {
    let t21473 = t2448 * t27 * t402;
    let t21474 = t5770 * t21473;
    let t21476 = t5783 * t21473;
    let t21477 = 2.923025_f64 * t21476;
    let t21479 = t360 * t110 * t7313;
    let t21501 = 2.0_f64 * t18755 + 11.75232_f64 * t18757 - 2.93808_f64 * t18759 - 8.81424_f64 * t21474 - t21477 - 3.0_f64 / 2.0_f64 * t21479 + 9.0_f64 / 2.0_f64 * t360 * t35 * t2233 * t2448 + 9.0_f64 / 2.0_f64 * t360 * t35 * t780 * t5980 + 3.0_f64 / 2.0_f64 * t360 * t35 * t7310 * t342 + 30.0_f64 * t360 * t35 * t7278 * t342 - 18.0_f64 * t360 * t35 * t2712 * t2209;
    (t21477, t21501)
}
