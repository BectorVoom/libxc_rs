//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 847/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk847(t6285: f64, t743: f64, t1308: f64, t571: f64, t2385: f64, t4763: f64, t6275: f64, t1319: f64, t1318: f64, t2146: f64, t2389: f64, t225: f64, t7337: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7815 = t6285 * t743;
    let t7816 = t1308 * t7815;
    let t7818 = 4.0_f64 / 15.0_f64 * t571 * t7816;
    let t7820 = 16.0_f64 / 15.0_f64 * t4763 * t2385;
    let t7821 = t6275 * t743;
    let t7822 = t1319 * t7821;
    let t7824 = 8.0_f64 / 15.0_f64 * t1318 * t7822;
    let t7826 = 8.0_f64 / 15.0_f64 * t2146 * t2389;
    let t7827 = t7337 * t225;
    (t7815, t7816, t7818, t7820, t7821, t7822, t7824, t7826, t7827)
}
