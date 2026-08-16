//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1425/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1425(t106951: f64, t510: f64, t652: f64, t1774: f64, t28017: f64, t1845: f64, t6347: f64, t22574: f64, t8643: f64, t28831: f64, t91655: f64, t1983: f64, t26167: f64, t28834: f64) -> (f64, f64, f64, f64, f64) {
    let t107496 = 2.0_f64 * t652 * t510 * t106951;
    let t107499 = 6.0_f64 * t652 * t1774 * t28017;
    let t107504 = t6347 * t1845;
    let t107507 = 9.0_f64 * t22574 * t8643 * t107504;
    let t107509 = 18.0_f64 * t91655 * t28831;
    let t107512 = 9.0_f64 * t1983 * t26167 * t28834;
    (t107496, t107499, t107507, t107509, t107512)
}
