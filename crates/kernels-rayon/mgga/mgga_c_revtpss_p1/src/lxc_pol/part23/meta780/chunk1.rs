//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2587/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2587(t59336: f64, t3655: f64, t5262: f64, t12966: f64, t1803: f64, t17235: f64, t372: f64, t1284: f64, t17306: f64, t3624: f64, t12898: f64, t1804: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t59337 = 0.7622047665434619906e-3_f64 * t59336;
    let t59338 = t5262 * t3655;
    let t59339 = 0.14291339372689912324e-3_f64 * t59338;
    let t59355 = t12966 * t1803;
    let t59362 = t372 * t17235;
    let t59411 = t17306 * t1284 * t3624;
    let t59419 = t1804 * t12898;
    (t59337, t59339, t59355, t59362, t59411, t59419)
}
