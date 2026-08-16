//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1042/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1042(t125: f64, t26439: f64, t86: f64, t748: f64, t7603: f64, t2526: f64, t754: f64, t2398: f64, t2720: f64, t2157: f64, t137: f64, t2425: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26441 = t86 * t125 * t26439;
    let t26444 = t86 * t748 * t7603;
    let t26446 = t754 * t2526;
    let t26448 = t86 * t125 * t26446;
    let t26450 = t2720 * t2398;
    let t26451 = t26450 * t2157;
    let t26454 = t86 * t2425 * t137;
    (t26441, t26444, t26446, t26448, t26450, t26451, t26454)
}
