//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 507/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk507(t2505: f64, t493: f64, t795: f64, t808: f64, t822: f64, t835: f64, t1371: f64, t2411: f64, t2415: f64, t589: f64, t2419: f64, t1346: f64, t1366: f64, t1941: f64, t2053: f64, t2413: f64, t2417: f64, t2421: f64, t25: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2507 = 4.0_f64 / 15.0_f64 * t493 * t2505;
    let t2509 = 4.0_f64 / 15.0_f64 * t795 * t808;
    let t2511 = 4.0_f64 / 15.0_f64 * t822 * t835;
    let t2517 = t1371 * t2411;
    let t2520 = t589 * t2415;
    let t2523 = t589 * t2419;
    let t2526 = t1346 + 0.023994444444444443_f64 * t1941 - 0.023994444444444443_f64 * t2413 + 0.07198333333333333_f64 * t2417 - 0.035991666666666665_f64 * t2421 + t1366 + 0.008888888888888889_f64 * t2053 - 0.0022222222222222222_f64 * t25 * t2517 + 0.013333333333333334_f64 * t25 * t2520 - 0.006666666666666667_f64 * t25 * t2523;
    (t2507, t2509, t2511, t2517, t2520, t2523, t2526)
}
