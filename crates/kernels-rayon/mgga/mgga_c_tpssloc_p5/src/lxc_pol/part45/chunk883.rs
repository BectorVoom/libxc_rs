//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 883/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk883(t31483: f64, t31517: f64, t113: f64, t1874: f64, t23938: f64, t26977: f64, t6525: f64, t7042: f64, t7217: f64, t8643: f64, t1983: f64, t1976: f64, t2036: f64, t31294: f64, t31296: f64, t31298: f64, t31302: f64, t31305: f64, t31306: f64, t6862: f64, t7040: f64) -> (f64, f64, f64) {
    let t31518 = t31483 + t31517;
    let t31519 = t113 * t31518;
    let t31521 = 2.0_f64 * t23938 * t1874;
    let t31523 = 2.0_f64 * t26977 * t1874;
    let t31525 = 2.0_f64 * t7042 * t6525;
    let t31526 = t7217 * t8643;
    let t31527 = t1983 * t31526;
    let t31528 = -t1976 * t7040 - t2036 * t6862 + t31294 - t31296 - t31298 - t31302 + t31305 + t31306 - t31519 - t31521 - t31523 - t31525 - t31527;
    (t31518, t31526, t31528)
}
