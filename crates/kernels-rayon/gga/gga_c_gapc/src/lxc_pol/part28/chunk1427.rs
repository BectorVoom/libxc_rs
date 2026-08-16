//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1427/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1427(t12295: f64, t987: f64, t36288: f64, t36290: f64, t36295: f64, t36299: f64, t36303: f64, t36304: f64, t36305: f64, t36312: f64, t36314: f64, t36318: f64, t36326: f64, t36453: f64, t36455: f64, t36462: f64, t36465: f64, t36479: f64, t36481: f64, t36893: f64, t37302: f64) -> f64 {
    let t38853 = t987 * t12295;
    let t38854 = t36288 + t36290 + t36295 + t36299 + t36303 - t36304 - t36305 + t36312 + t36314 + t36318 + t36326 + t36453 - t36455 + t36462 + t36465 + t38853 + t36479 - t36481 - t36893 - t37302;
    t38854
}
