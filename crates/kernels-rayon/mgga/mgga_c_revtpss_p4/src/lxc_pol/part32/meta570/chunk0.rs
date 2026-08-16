//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1894/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1894(t26169: f64, t7702: f64, t28640: f64, t6954: f64, t1923: f64, t28089: f64, t7348: f64, t26205: f64, t26204: f64, t7719: f64, t101214: f64, t2047: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t101901 = 16.0_f64 / 9.0_f64 * t7702 * t26169;
    let t101903 = 16.0_f64 / 9.0_f64 * t6954 * t28640;
    let t101906 = 16.0_f64 / 9.0_f64 * t1923 * t7348 * t28089;
    let t101907 = t7702 * t26205;
    let t101929 = t1923 * t26204 * t7719;
    let t101935 = t2047 * t101214;
    (t101901, t101903, t101906, t101907, t101929, t101935)
}
