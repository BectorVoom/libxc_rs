//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 582/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk582(t1165: f64, t3253: f64, t3457: f64, t3456: f64, t1172: f64, t1530: f64) -> (f64, f64, f64) {
    let t3459 = t1165 * t3253 * t3457;
    let t3460 = t3456 * t3459;
    let t3462 = t1530 * t1172;
    (t3459, t3460, t3462)
}
