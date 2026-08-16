//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 945/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk945(t2611: f64, t3992: f64, t2620: f64, t2623: f64, t1: f64, t283: f64, t4027: f64, t4047: f64, t807: f64, t1381: f64, t2838: f64, t5042: f64, t912: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14885 = t3992 * t2611;
    let t14890 = t3992 * t2620;
    let t14892 = t3992 * t2623;
    let t14898 = t4027 * t1 * t283;
    let t14900 = t4047 * t807;
    let t14902 = t1381 * t2838;
    let t14904 = t5042 * t912;
    (t14885, t14890, t14892, t14898, t14900, t14902, t14904)
}
