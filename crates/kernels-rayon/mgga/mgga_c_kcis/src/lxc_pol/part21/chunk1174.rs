//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1174/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1174(t2810: f64, t10462: f64, t975: f64, t10461: f64, t278: f64, t299: f64, t2835: f64, t3038: f64, t3323: f64, t3329: f64, t10497: f64, t1138: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31296 = t2810 * t2810;
    let t31297 = 1.0_f64 / t31296;
    let t32896 = t975 * t10462;
    let t33822 = t278 / t10461 / t299;
    let t33827 = t3038 * t2835;
    let t33848 = t3323 * t3329;
    let t33853 = t1138 * t10497;
    (t31297, t32896, t33822, t33827, t33848, t33853)
}
