//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1124/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1124(t1558: f64, t30644: f64, t4326: f64, t7647: f64, t1421: f64, t1983: f64, t30827: f64, t7586: f64, t1545: f64, t31824: f64, t1416: f64, t1992: f64, t30154: f64) -> (f64, f64, f64, f64, f64) {
    let t35979 = t30644 * t1558;
    let t35981 = t7647 * t4326;
    let t35985 = t30827 * t7586 * t1983 * t1421;
    let t35987 = t31824 * t1545;
    let t35991 = t30154 * t7586 * t1992 * t1416;
    (t35979, t35981, t35985, t35987, t35991)
}
