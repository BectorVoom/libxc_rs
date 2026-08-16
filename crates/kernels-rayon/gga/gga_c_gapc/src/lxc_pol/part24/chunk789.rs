//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 789/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk789(t9418: f64, t9419: f64, t2316: f64, t2982: f64, t3391: f64, t2300: f64, t3387: f64, t7927: f64, t876: f64, t3378: f64, t3367: f64, t3383: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9420 = t9418 * t9419;
    let t9422 = t2982 * t2316;
    let t9423 = t3391 * t9422;
    let t9425 = t2982 * t2300;
    let t9426 = t3387 * t9425;
    let t9429 = t7927 * t876;
    let t9430 = t3378 * t9429;
    let t9432 = t3367 * t3383;
    (t9420, t9422, t9423, t9425, t9426, t9430, t9432)
}
