//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 951/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk951(t1049: f64, t442: f64, t13964: f64, t12951: f64, t167: f64, t1391: f64, t3278: f64, t3532: f64, t967: f64, t143: f64, t3283: f64, t443: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14082 = t1049 * t442;
    let t14083 = 0.62154466893555682512e-3_f64 * t14082;
    let t14084 = 0.71734315950379065738e-1_f64 * t13964;
    let t14085 = t167 * t12951;
    let t14088 = t1391 * t3278;
    let t14090 = t967 * t3532;
    let t14091 = t14090 * t3278;
    let t14093 = t143 * t3532;
    let t14096 = t443 * t3283;
    (t14083, t14084, t14085, t14088, t14091, t14093, t14096)
}
