//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1055/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1055(t14545: f64, t31272: f64, t21315: f64, t8241: f64, t30205: f64, t381: f64, t498: f64, t493: f64, t21066: f64, t8268: f64, t30962: f64, t6317: f64) -> (f64, f64, f64, f64, f64) {
    let t31273 = t14545 * t31272;
    let t31275 = t21315 * t8241;
    let t31277 = t381 * t30205;
    let t31278 = t498 * t31277;
    let t31279 = t493 * t31278;
    let t31281 = t21066 * t8268;
    let t31283 = t6317 * t30962;
    (t31273, t31275, t31279, t31281, t31283)
}
