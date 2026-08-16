//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1012/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1012(t1451: f64, t7605: f64, t1423: f64, t7736: f64, t30318: f64, t542: f64, t2327: f64, t7630: f64, t13287: f64, t31057: f64, t35700: f64, t1429: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35736 = t7605 * t1451;
    let t35737 = 0.34299214494455789578e-2_f64 * t35736;
    let t35738 = t7736 * t1423;
    let t35740 = t30318 * t542;
    let t35744 = t7630 * t2327;
    let t35747 = t31057 * t13287 * t35700;
    let t35748 = 0.42874018118069736972e-3_f64 * t35747;
    let t35755 = t7605 * t1429;
    (t35737, t35738, t35740, t35744, t35748, t35755)
}
