//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2537/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2537(t11466: f64, t300: f64, t51973: f64, t52035: f64, t52037: f64, t1633: f64, t3012: f64, t2986: f64, t4682: f64, t11465: f64, t1626: f64, t11509: f64, t4707: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t52238 = t300 * t11466;
    let t52337 = 0.12361111111111111111e-1_f64 * t51973;
    let t52346 = 0.24722222222222222222e-1_f64 * t52035;
    let t52397 = 0.2283111111111111111e-1_f64 * t51973;
    let t52406 = 0.4566222222222222222e-1_f64 * t52035;
    let t52407 = 0.1522074074074074074e-1_f64 * t52037;
    let t52430 = t3012 * t1633;
    let t52440 = t4682 * t2986;
    let t52443 = t1626 * t11465;
    let t52459 = t4707 * t11509;
    (t52238, t52337, t52346, t52397, t52406, t52407, t52430, t52440, t52443, t52459)
}
