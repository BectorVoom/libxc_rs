//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 691/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk691(t670: f64, t7330: f64, t572: f64, t117: f64, t7002: f64, t1461: f64, t2040: f64, t573: f64, t7324: f64, t7329: f64, t38: f64, t4173: f64) -> (f64, f64, f64, f64) {
    let t7331 = t7330 * t670;
    let t7333 = 6.0_f64 * t572 * t7331;
    let t7334 = t117 * t7002;
    let t7336 = 3.0_f64 * t572 * t7334;
    let t7337 = 3.0_f64 * t1461 * t2040 + t573 * t7324 + t7329 + t7333 + t7336;
    let t7702 = t4173 * t38;
    (t7331, t7334, t7337, t7702)
}
