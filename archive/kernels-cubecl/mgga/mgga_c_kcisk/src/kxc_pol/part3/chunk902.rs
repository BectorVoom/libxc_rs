//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 902/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk902<F: Float>(t13456: F, t5625: F, t3484: F, t3482: F, t1329: F, t13413: F, t13417: F, t13420: F, t13426: F, t13429: F, t13433: F, t13437: F, t13441: F, t13448: F, t13454: F, t3491: F, t3925: F, t4159: F) -> (F, F) {
    let t13457 = t5625 * t13456;
    let t13458 = t3484 * t13457;
    let t13459 = t3482 * t13458;
    let t13461 = -F::cast_from(0.2653111111111111111e-1_f64) * t13413 - F::cast_from(0.13265555555555555555e-1_f64) * t13417 + F::cast_from(0.49745833333333333332e-2_f64) * t13420 + F::cast_from(0.16581944444444444444e-2_f64) * t13426 + F::cast_from(0.8290972222222222222e-2_f64) * t13429 + F::cast_from(0.16581944444444444444e-2_f64) * t13433 - F::cast_from(0.43134342e-1_f64) * t13437 * t13441 - F::cast_from(0.579e0_f64) * t3491 * t4159 + F::cast_from(0.579e0_f64) * t3491 * t3925 - F::cast_from(0.579e0_f64) * t13448 * t1329 + F::cast_from(0.33163888888888888887e-2_f64) * t13454 - F::cast_from(0.99491666666666666664e-2_f64) * t13459;
    (t13459, t13461)
}
