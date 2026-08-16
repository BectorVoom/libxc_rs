//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 497/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk497(t1313: f64, t2030: f64, t519: f64, t549: f64, t816: f64, t1319: f64, t1318: f64, t1451: f64, t1477: f64, t1516: f64, t1629: f64, t1632: f64, t1637: f64, t1641: f64, t1994: f64, t1999: f64, t2004: f64, t2009: f64, t2013: f64, t2016: f64, t2020: f64, t2025: f64, t2029: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2031 = t1313 * t2030;
    let t2033 = 4.0_f64 / 45.0_f64 * t519 * t2031;
    let t2034 = t816 * t549;
    let t2035 = t1319 * t2034;
    let t2037 = 8.0_f64 / 45.0_f64 * t1318 * t2035;
    let t2039 = 8.0_f64 / 135.0_f64 * t1451;
    let t2040 = 8.0_f64 / 135.0_f64 * t1477;
    let t2041 = 4.0_f64 / 45.0_f64 * t1516;
    let t2042 = t1994 - t1999 + t2004 + t2009 - t2013 - t2016 + t2020 - t2025 + t2029 - t2033 + t2037 + t1629 + 0.10821041362364843_f64 * t1632 + t1637 + t1641 + t2039 + t2040 + t2041;
    (t2031, t2033, t2034, t2035, t2037, t2039, t2040, t2041, t2042)
}
