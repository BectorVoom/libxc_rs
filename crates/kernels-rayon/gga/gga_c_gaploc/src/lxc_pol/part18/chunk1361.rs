//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1361/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1361(t1441: f64, t34267: f64, t590: f64, t30247: f64, t30251: f64, t30253: f64, t30261: f64, t30263: f64, t30265: f64, t30288: f64, t30294: f64, t34256: f64, t34258: f64, t34260: f64, t34261: f64, t34262: f64, t34263: f64, t34266: f64) -> f64 {
    let t34270 = 0.2044956050875773316e1_f64 * t1441 * t34267 * t590;
    let t34271 = t34256 + t34258 - t30247 - t30251 + t30253 - t30261 - t34260 + t30263 - t30265 - t30288 + t30294 + t34261 + t34262 + t34263 + t34266 + t34270;
    t34271
}
