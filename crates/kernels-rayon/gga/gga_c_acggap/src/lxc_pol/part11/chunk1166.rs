//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1166/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1166(t1470: f64, t30540: f64, t1549: f64, t30644: f64, t1554: f64, t1558: f64, t4326: f64, t7647: f64, t1421: f64, t1983: f64, t30827: f64, t7586: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35973 = t30540 * t1470;
    let t35975 = t30644 * t1549;
    let t35976 = 0.17149607247227894789e-2_f64 * t35975;
    let t35977 = t30644 * t1554;
    let t35978 = 0.17149607247227894789e-2_f64 * t35977;
    let t35979 = t30644 * t1558;
    let t35980 = 0.85748036236139473944e-3_f64 * t35979;
    let t35981 = t7647 * t4326;
    let t35982 = 0.85748036236139473944e-3_f64 * t35981;
    let t35985 = t30827 * t7586 * t1983 * t1421;
    (t35973, t35976, t35978, t35980, t35982, t35985)
}
