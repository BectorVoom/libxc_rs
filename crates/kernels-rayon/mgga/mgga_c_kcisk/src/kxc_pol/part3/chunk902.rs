//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 902/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk902(t13456: f64, t5625: f64, t3484: f64, t3482: f64, t1329: f64, t13413: f64, t13417: f64, t13420: f64, t13426: f64, t13429: f64, t13433: f64, t13437: f64, t13441: f64, t13448: f64, t13454: f64, t3491: f64, t3925: f64, t4159: f64) -> (f64, f64) {
    let t13457 = t5625 * t13456;
    let t13458 = t3484 * t13457;
    let t13459 = t3482 * t13458;
    let t13461 = -0.2653111111111111111e-1_f64 * t13413 - 0.13265555555555555555e-1_f64 * t13417 + 0.49745833333333333332e-2_f64 * t13420 + 0.16581944444444444444e-2_f64 * t13426 + 0.8290972222222222222e-2_f64 * t13429 + 0.16581944444444444444e-2_f64 * t13433 - 0.43134342e-1_f64 * t13437 * t13441 - 0.579e0_f64 * t3491 * t4159 + 0.579e0_f64 * t3491 * t3925 - 0.579e0_f64 * t13448 * t1329 + 0.33163888888888888887e-2_f64 * t13454 - 0.99491666666666666664e-2_f64 * t13459;
    (t13459, t13461)
}
