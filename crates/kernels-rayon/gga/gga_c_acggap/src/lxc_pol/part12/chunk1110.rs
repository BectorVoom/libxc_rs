//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1110/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1110(t13287: f64, t31195: f64, t35749: f64, t2001: f64, t4724: f64, t1429: f64, t7605: f64, t1165: f64, t20590: f64, t604: f64, t7337: f64, t5272: f64, t7561: f64) -> (f64, f64, f64, f64, f64) {
    let t35751 = t31195 * t13287 * t35749;
    let t35753 = t2001 * t4724;
    let t35755 = t7605 * t1429;
    let t35759 = t7337 * t1165 * t604 * t20590;
    let t35766 = t7561 * t5272;
    (t35751, t35753, t35755, t35759, t35766)
}
