//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 891/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk891(t1177: f64, t13263: f64, t839: f64, t944: f64, t3206: f64, t366: f64, t374: f64, t1145: f64, t3570: f64, t1117: f64, t1121: f64, t3573: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13264 = t13263 * t1177;
    let t13268 = t944 * t839;
    let t13273 = t3206 * t366;
    let t13274 = t13273 * t374;
    let t13276 = t3570 * t1145;
    let t13278 = t3570 * t1117;
    let t13280 = t3573 * t1121;
    (t13264, t13268, t13273, t13274, t13276, t13278, t13280)
}
