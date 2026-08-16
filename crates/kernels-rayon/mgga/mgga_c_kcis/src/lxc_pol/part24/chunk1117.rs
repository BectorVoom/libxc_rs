//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1117/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1117(t2531: f64, t2537: f64, t2810: f64, t10462: f64, t975: f64, t10461: f64, t278: f64, t299: f64, t10497: f64, t1138: f64, t10496: f64, t364: f64, t392: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31274 = t2531 * t2537;
    let t31296 = t2810 * t2810;
    let t31297 = 1.0_f64 / t31296;
    let t32896 = t975 * t10462;
    let t33822 = t278 / t10461 / t299;
    let t33853 = t1138 * t10497;
    let t33862 = t364 / t10496 / t392;
    (t31274, t31297, t32896, t33822, t33853, t33862)
}
