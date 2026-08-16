//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 706/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk706(t1459: f64, t2042: f64, t116: f64, t1936: f64, t670: f64, t572: f64, t117: f64, t7002: f64, t1461: f64, t2040: f64, t573: f64, t7324: f64) -> (f64, f64, f64, f64) {
    let t7329 = 3.0_f64 * t1459 * t2042;
    let t7330 = t116 * t1936;
    let t7331 = t7330 * t670;
    let t7333 = 6.0_f64 * t572 * t7331;
    let t7334 = t117 * t7002;
    let t7336 = 3.0_f64 * t572 * t7334;
    let t7337 = 3.0_f64 * t1461 * t2040 + t573 * t7324 + t7329 + t7333 + t7336;
    (t7330, t7331, t7334, t7337)
}
