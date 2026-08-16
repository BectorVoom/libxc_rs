//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1176/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1176(t1137: f64, t5910: f64, t1163: f64, t1181: f64, t1532: f64, t1753: f64, t879: f64, t14176: f64, t5732: f64, t506: f64, t955: f64, t3431: f64, t5712: f64) -> (f64, f64, f64, f64, f64) {
    let t21331 = t1137 * t5910;
    let t21338 = t1163 * t1181 * t1532 * t1753 * t879;
    let t21340 = t14176 * t5732;
    let t21342 = t955 * t506;
    let t21348 = t3431 * t5712;
    (t21331, t21338, t21340, t21342, t21348)
}
