//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1242/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1242(t13272: f64, t32596: f64, t8623: f64, t32589: f64, t121629: f64, t34177: f64, t1493: f64, t1925: f64, t119457: f64, t644: f64, t1497: f64, t8442: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t128385 = t13272 * t32596 * t8623;
    let t128394 = t13272 * t32589;
    let t128399 = t121629 * t34177;
    let t128401 = t1925 * t1493;
    let t128403 = t119457 * t128401 * t644;
    let t128409 = t1925 * t1497;
    let t128411 = t8442 * t128409 * t644;
    (t128385, t128394, t128399, t128403, t128409, t128411)
}
