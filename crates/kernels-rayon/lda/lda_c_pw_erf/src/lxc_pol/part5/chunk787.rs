//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 787/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk787(t230: f64, t2660: f64, t4595: f64, t4718: f64, t4719: f64, t6578: f64, t6582: f64, t6584: f64, t6586: f64, t6588: f64, t6594: f64, t6599: f64, t6603: f64, t6605: f64, t6606: f64, t6613: f64, t6633: f64, t6673: f64) -> (f64, f64) {
    let t7256 = t2660 * t230;
    let t7258 = -t6578 + t6582 + t6584 + t6586 + t6588 + t6594 + t6599 + t6603 + t6605 - t6606 - t4595 + t6613 + t6633 + t6673 + t4718 + 8.0_f64 / 3.0_f64 * t4719 + 4.0_f64 / 3.0_f64 * t7256;
    (t7256, t7258)
}
