//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1162/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1162(t2029: f64, t7002: f64, t2022: f64, t7020: f64, t1395: f64, t8509: f64, t31288: f64, t576: f64, t112: f64, t31253: f64, t111: f64, t8496: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t114439 = t7002 * t2029;
    let t114441 = t2022 * t7020;
    let t114449 = t1395 * t8509;
    let t114451 = t576 * t31288;
    let t114475 = t31253 * t112;
    let t114495 = t8496 * t111;
    (t114439, t114441, t114449, t114451, t114475, t114495)
}
