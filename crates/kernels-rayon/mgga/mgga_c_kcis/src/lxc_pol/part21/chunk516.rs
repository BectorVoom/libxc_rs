//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 516/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk516(t1045: f64, t922: f64, t3274: f64, t1071: f64, t347: f64, t1103: f64, t2630: f64, t1104: f64, t2635: f64, t932: f64, t2944: f64, t345: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3275 = t922 * t1045;
    let t3276 = t3274 * t3275;
    let t3279 = t347 * t1071;
    let t3281 = t1103 * t3279 * t2630;
    let t3285 = t1103 * t1104 * t2635;
    let t3288 = t932 * t347;
    let t3289 = t3288 * t2944;
    let t3290 = t345 * t3289;
    (t3275, t3276, t3281, t3285, t3288, t3289, t3290)
}
