//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1364/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1364(t2469: f64, t2822: f64, t36303: f64, t36304: f64, t36305: f64, t36307: f64, t36309: f64, t36312: f64, t36314: f64, t36316: f64, t36318: f64, t36320: f64, t36323: f64, t36324: f64, t36326: f64, t36331: f64, t36453: f64, t36455: f64, t36457: f64, t3846: f64) -> f64 {
    let t36458 = 2.0_f64 * t2469 * t2822 * t3846 - t36303 + t36304 + t36305 + t36307 + t36309 - t36312 - t36314 - t36316 - t36318 + t36320 + t36323 - t36324 - t36326 + t36331 - t36453 + t36455 + t36457;
    t36458
}
