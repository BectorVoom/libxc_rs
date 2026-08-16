//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 857/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk857(t1164: f64, t5679: f64, t174: f64, t507: f64, t435: f64, t1449: f64, t322: f64, t1460: f64, t1444: f64, t1421: f64, t301: f64, t1439: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20417 = t1164 * t5679;
    let t20555 = t507 * t174;
    let t20559 = t507 * t435;
    let t20992 = t1449 * t322;
    let t21118 = t1460 * t322;
    let t21143 = t1444 * t322;
    let t21955 = t1421 * t301;
    let t22040 = t1439 * t322;
    (t20417, t20555, t20559, t20992, t21118, t21143, t21955, t22040)
}
