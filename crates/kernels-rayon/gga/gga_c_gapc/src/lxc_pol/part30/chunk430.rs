//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 430/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk430(t61: f64, t760: f64, t798: f64, t2224: f64, t793: f64, t435: f64, t818: f64) -> (f64, f64, f64, f64) {
    let t2261 = t61 * t760;
    let t2262 = t2261 * t798;
    let t2265 = t2224 * t793;
    let t2268 = t435 * t818;
    (t2261, t2262, t2265, t2268)
}
