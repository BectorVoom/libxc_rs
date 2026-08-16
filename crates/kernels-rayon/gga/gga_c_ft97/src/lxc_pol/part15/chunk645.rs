//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 645/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk645(t458: f64, t4772: f64, t1775: f64, t4762: f64, t2112: f64, t358: f64, t16925: f64, t16928: f64, t1882: f64, t4819: f64, t4815: f64, t4790: f64, t604: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17281 = t458 * t4772;
    let t17310 = t1775 * t4762;
    let t17338 = t2112 * t358;
    let t17351 = t16925 / 3.0_f64;
    let t17352 = 2.0_f64 / 3.0_f64 * t16928;
    let t17360 = t1882 * t4819;
    let t17362 = t1882 * t4815;
    let t17409 = t4790 * t604;
    (t17281, t17310, t17338, t17351, t17352, t17360, t17362, t17409)
}
