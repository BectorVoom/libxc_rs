//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 843/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk843(t1268: f64, t7639: f64, t7643: f64, t538: f64, t7647: f64, t7651: f64, t7655: f64, t3516: f64, t7635: f64, t25: f64, t3472: f64, t3543: f64, t4600: f64, t7641: f64, t7645: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7758 = t1268 * t7639;
    let t7761 = t1268 * t7643;
    let t7764 = t538 * t7647;
    let t7767 = t538 * t7651;
    let t7770 = t538 * t7655;
    let t7773 = t3516 * t7635;
    let t7779 = 0.013333333333333334_f64 * t25 * t7758 - 0.006666666666666667_f64 * t25 * t7761 - 0.04_f64 * t25 * t7764 + 0.04_f64 * t25 * t7767 - 0.006666666666666667_f64 * t25 * t7770 - 0.002962962962962963_f64 * t25 * t7773 - t3472 - 0.047988888888888886_f64 * t4600 - t3543 + 0.14396666666666666_f64 * t7641 - 0.07198333333333333_f64 * t7645;
    (t7758, t7761, t7764, t7767, t7770, t7773, t7779)
}
