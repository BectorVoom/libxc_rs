//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1115/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1115(t30318: f64, t537: f64, t1165: f64, t2068: f64, t34681: f64, t8600: f64, t7433: f64, t8908: f64, t8912: f64, t1181: f64, t35618: f64, t599: f64, t7337: f64) -> (f64, f64, f64, f64, f64) {
    let t35829 = t30318 * t537;
    let t35833 = t2068 * t1165 * t8600 * t34681;
    let t35835 = t7433 * t8908;
    let t35837 = t7433 * t8912;
    let t35841 = t7337 * t1181 * t599 * t35618;
    (t35829, t35833, t35835, t35837, t35841)
}
