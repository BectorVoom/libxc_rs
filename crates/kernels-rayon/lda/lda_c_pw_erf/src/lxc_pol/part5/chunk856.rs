//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 856/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk856(t7923: f64, t7948: f64, t2634: f64, t774: f64, t2642: f64, t2610: f64, t756: f64, t133: f64, t1870: f64, t1871: f64, t3280: f64, t3284: f64, t3322: f64, t3348: f64, t5660: f64, t7203: f64, t7205: f64, t7893: f64, t7896: f64, t7915: f64, t7920: f64, t7926: f64, t7935: f64, t7940: f64, t7941: f64, t7947: f64) -> (f64, f64, f64, f64, f64) {
    let t7949 = t7923 + t7948;
    let t7957 = t2634 * t774;
    let t7960 = t774 * t2642;
    let t7970 = t756 * t2610;
    let t7974 = 1.724255_f64 * t7203 - 5.172765_f64 * t7205 - t3348 - t3284 + t3280 - t7926 - t3322 - t7940 + t7941 - 1.724255_f64 * t133 * t7915 - t7935 + t7947 - 2.2990066666666666_f64 * t5660 - t7893 + t7896 - 20.69106_f64 * t133 * t7920 + 15.518295_f64 * t1870 * t1871 * t7970;
    (t7949, t7957, t7960, t7970, t7974)
}
