//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 480/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk480(t2349: f64, t531: f64, t1589: f64, t888: f64, t1628: f64, t907: f64, t590: f64, t1407: f64, t914: f64, t1225: f64, t124: f64, t1390: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2446 = t531 * t2349;
    let t2449 = t1589 * t888;
    let t2452 = t1628 * t907;
    let t2457 = t888 * t590;
    let t2460 = t1407 * t914;
    let t2462 = 1.0_f64 / t1225;
    let t2464 = t2462 * t124 * t1390;
    (t2446, t2449, t2452, t2457, t2460, t2462, t2464)
}
