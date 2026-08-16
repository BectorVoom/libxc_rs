//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 712/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk712(t1487: f64, t301: f64, t1089: f64, t368: f64, t372: f64, t1083: f64, t398: f64, t1539: f64, t360: f64) -> (f64, f64, f64, f64, f64) {
    let t5111 = t1487 * t301;
    let t5113 = t1089 * t368 * t5111;
    let t5116 = t1487 * t372;
    let t5118 = t398 * t1083 * t5116;
    let t5122 = t1539 * t360;
    (t5111, t5113, t5116, t5118, t5122)
}
