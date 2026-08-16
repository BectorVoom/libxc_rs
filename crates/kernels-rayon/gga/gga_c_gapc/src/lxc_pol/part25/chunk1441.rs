//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1441/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1441(t3909: f64, t4905: f64, t12625: f64, t36326: f64, t36331: f64, t36455: f64, t36457: f64, t36460: f64, t36462: f64, t36467: f64, t36470: f64, t36472: f64, t36474: f64, t36483: f64, t36892: f64, t36894: f64, t37308: f64, t38708: f64, t38710: f64, t7056: f64) -> (f64, f64) {
    let t38834 = t4905 * t3909;
    let t38835 = 4.0_f64 * t12625 * t7056 - t36326 + t36331 + t36455 + t36457 - t36460 - t36462 + t36467 - t36470 - t36472 - t36474 + t36483 - t36892 - t36894 - t37308 + t38708 - t38710 + t38834;
    (t38834, t38835)
}
