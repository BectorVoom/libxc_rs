//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 737/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk737(t11: f64, t6662: f64, t558: f64, t6005: f64, t557: f64, t3627: f64, t4013: f64, t4657: f64, t4659: f64, t4662: f64, t4663: f64, t6638: f64, t6641: f64, t6644: f64, t6647: f64, t6649: f64, t6652: f64, t6655: f64, t6657: f64, t6660: f64) -> (f64, f64, f64, f64, f64) {
    let t6663 = t11 * t6662;
    let t6665 = t558 * t6005;
    let t6666 = t557 * t6665;
    let t6667 = t11 * t6666;
    let t6669 = t4013 + 0.0008396296296296296_f64 * t3627 + 0.0016792592592592592_f64 * t4657 - 0.0008396296296296296_f64 * t4659 + t4662 + 0.002518888888888889_f64 * t4663 - 0.0004198148148148148_f64 * t6638 + 0.002099074074074074_f64 * t6641 - 0.007556666666666666_f64 * t6644 - 0.005037777777777778_f64 * t6647 + 0.0012594444444444445_f64 * t6649 + 0.011335_f64 * t6652 + 0.015113333333333333_f64 * t6655 - 0.0006297222222222223_f64 * t6657 + 0.0012594444444444445_f64 * t6660 - 0.003778333333333333_f64 * t6663 + 0.0018891666666666666_f64 * t6667;
    (t6663, t6665, t6666, t6667, t6669)
}
