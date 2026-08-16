//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1301/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1301(t30299: f64, t30305: f64, t10600: f64, t1415: f64, t20902: f64, t31585: f64, t493: f64, t1441: f64, t590: f64, t31590: f64, t2897: f64, t4398: f64, t7030: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34262 = 0.31952438294933958064e-1_f64 * t30299;
    let t34263 = 0.63904876589867916128e-1_f64 * t30305;
    let t34264 = t1415 * t10600;
    let t34266 = 0.79445533226334281486e-1_f64 * t34264 * t20902;
    let t34267 = t493 * t31585;
    let t34270 = 0.2044956050875773316e1_f64 * t1441 * t34267 * t590;
    let t34273 = t493 * t31590;
    let t34276 = 0.2044956050875773316e1_f64 * t1441 * t34273 * t590;
    let t34278 = t4398 * t2897 * t7030;
    (t34262, t34263, t34266, t34270, t34276, t34278)
}
