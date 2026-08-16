//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 640/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk640(t1165: f64, t3457: f64, t5852: f64, t3456: f64, t1772: f64, t322: f64, t368: f64, t398: f64, t384: f64, t1795: f64, t372: f64, t1459: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6184 = t1165 * t5852 * t3457;
    let t6185 = t3456 * t6184;
    let t6192 = t1772 * t322;
    let t6194 = t398 * t368 * t6192;
    let t6195 = t384 * t6194;
    let t6198 = t1795 * t372;
    let t6200 = t398 * t1459 * t6198;
    (t6184, t6185, t6192, t6194, t6195, t6198, t6200)
}
