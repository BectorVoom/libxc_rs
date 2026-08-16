//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1109/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1109(t1423: f64, t7736: f64, t30318: f64, t542: f64, t2001: f64, t4886: f64, t2327: f64, t7630: f64, t13287: f64, t31057: f64, t35700: f64, t2288: f64, t3196: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35738 = t7736 * t1423;
    let t35740 = t30318 * t542;
    let t35742 = t2001 * t4886;
    let t35744 = t7630 * t2327;
    let t35747 = t31057 * t13287 * t35700;
    let t35749 = t2288 * t3196;
    (t35738, t35740, t35742, t35744, t35747, t35749)
}
