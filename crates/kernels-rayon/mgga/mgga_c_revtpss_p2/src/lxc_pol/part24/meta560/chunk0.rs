//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1683/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1683(t19049: f64, t6223: f64, t11465: f64, t88008: f64, t973: f64, t981: f64, t23696: f64, t4719: f64, t6227: f64, t300: f64, t88477: f64, t23457: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t88580 = 0.35089341735807877242e1_f64 * t19049 * t6223;
    let t88584 = 0.14035736694323150897e2_f64 * t981 * t11465 * t88008 * t973;
    let t88586 = 0.23392894490538584828e1_f64 * t4719 * t23696;
    let t88588 = 0.10389515463408878255e3_f64 * t19049 * t6227;
    let t88590 = 0.19751673498613801407e-1_f64 * t300 * t88477;
    let t88592 = 0.14035736694323150897e2_f64 * t4719 * t23457;
    (t88580, t88584, t88586, t88588, t88590, t88592)
}
