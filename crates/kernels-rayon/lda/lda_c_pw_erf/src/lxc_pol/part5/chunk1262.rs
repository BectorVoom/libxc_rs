//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1262/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1262(t7016: f64, t795: f64, t185: f64, t514: f64, t7793: f64, t511: f64, t7794: f64, t331: f64, t7770: f64, t7773: f64, t20007: f64, t504: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22630 = t795 * t7016;
    let t22631 = 4.0_f64 / 15.0_f64 * t22630;
    let t22633 = t185 * t514 * t7793;
    let t22634 = 4.0_f64 / 45.0_f64 * t22633;
    let t22636 = 2.0_f64 / 15.0_f64 * t511 * t7794;
    let t22649 = t331 * t7770;
    let t22651 = t331 * t7773;
    let t22653 = t504 * t20007;
    (t22631, t22634, t22636, t22649, t22651, t22653)
}
