//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2347/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2347(t12984: f64, t12998: f64, t5544: f64, t686: f64, t20933: f64, t2563: f64, t20923: f64, t41011: f64, t118: f64, t20756: f64, t41170: f64, t794: f64) -> (f64, f64, f64, f64) {
    let t68110 = t12998 * t686 * t12984 * t5544;
    let t68116 = t2563 * t20933;
    let t68118 = t41011 * t20923;
    let t68122 = t41170 * t118 * t794 * t20756;
    (t68110, t68116, t68118, t68122)
}
