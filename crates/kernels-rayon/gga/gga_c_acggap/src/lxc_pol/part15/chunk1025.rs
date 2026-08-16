//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1025/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1025(t1470: f64, t30540: f64, t1549: f64, t30644: f64, t1554: f64, t1558: f64, t4326: f64, t7647: f64, t1421: f64, t1983: f64, t30827: f64, t7586: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35973 = t30540 * t1470;
    let t35975 = t30644 * t1549;
    let t35977 = t30644 * t1554;
    let t35979 = t30644 * t1558;
    let t35981 = t7647 * t4326;
    let t35985 = t30827 * t7586 * t1983 * t1421;
    (t35973, t35975, t35977, t35979, t35981, t35985)
}
