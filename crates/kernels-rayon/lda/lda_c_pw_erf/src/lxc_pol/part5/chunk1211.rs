//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1211/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1211(t18188: f64, t2006: f64, t3965: f64, t16657: f64, t1996: f64, t4488: f64, t17883: f64, t17886: f64, t1308: f64, t571: f64, t593: f64, t7422: f64) -> (f64, f64, f64, f64, f64) {
    let t21885 = 8.0_f64 / 15.0_f64 * t3965 * t18188 * t2006;
    let t21888 = 8.0_f64 / 15.0_f64 * t4488 * t16657 * t1996;
    let t21889 = 16.0_f64 / 27.0_f64 * t17883;
    let t21890 = 16.0_f64 / 45.0_f64 * t17886;
    let t21894 = 8.0_f64 / 15.0_f64 * t571 * t1308 * t7422 * t593;
    (t21885, t21888, t21889, t21890, t21894)
}
