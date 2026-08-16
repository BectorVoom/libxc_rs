//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 778/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk778(t3: f64, t7318: f64, t1459: f64, t2042: f64, t116: f64, t1936: f64, t670: f64, t572: f64, t117: f64, t7002: f64, t1461: f64, t2040: f64, t573: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7319 = t3 * t7318;
    let t7324 = param_d * t7318;
    let t7329 = 3.0_f64 * t1459 * t2042;
    let t7330 = t116 * t1936;
    let t7331 = t7330 * t670;
    let t7333 = 6.0_f64 * t572 * t7331;
    let t7334 = t117 * t7002;
    let t7336 = 3.0_f64 * t572 * t7334;
    let t7337 = 3.0_f64 * t1461 * t2040 + t573 * t7324 + t7329 + t7333 + t7336;
    (t7319, t7324, t7330, t7331, t7334, t7337)
}
