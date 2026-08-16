//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 565/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk565(t3272: f64, t3273: f64, t1092: f64, t962: f64, t191: f64, t761: f64, t1093: f64, t2502: f64, t906: f64, t904: f64, t1045: f64, t291: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3274 = t3272 * t3273;
    let t3276 = t1092 * t962;
    let t3278 = t761 * t191;
    let t3279 = t3278 * t1093;
    let t3281 = t2502 * t906;
    let t3282 = t904 * t3281;
    let t3284 = t1045 * t291;
    (t3274, t3276, t3278, t3279, t3281, t3282, t3284)
}
