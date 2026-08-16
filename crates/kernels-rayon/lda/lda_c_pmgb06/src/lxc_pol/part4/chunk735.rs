//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 735/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk735(t1586: f64, t838: f64, t1380: f64, t493: f64, t2862: f64, t224: f64, t4622: f64, t4624: f64, t4626: f64, t4628: f64, t4630: f64, t4684: f64, t4713: f64, t4717: f64, t4718: f64, t4721: f64, t4723: f64, t4725: f64, t4727: f64, t4730: f64) -> (f64, f64, f64, f64, f64) {
    let t4731 = t838 * t1586;
    let t4732 = t1380 * t4731;
    let t4734 = t493 * t4732 / 45.0_f64;
    let t4735 = 4.0_f64 / 135.0_f64 * t2862;
    let t4736 = t4622 + t4624 + t4626 + t4628 + t4630 + t4684 - t4713 * t224 / 15.0_f64 - t4717 + 2.0_f64 / 135.0_f64 * t4718 - t4721 - t4723 + t4725 - t4727 - t4730 - t4734 - t4735;
    (t4731, t4732, t4734, t4735, t4736)
}
