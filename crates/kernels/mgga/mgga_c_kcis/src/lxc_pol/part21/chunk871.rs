//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 871/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk871<F: Float>(t13410: F, t13411: F, t3200: F, t13367: F, t13370: F, t13373: F, t13377: F, t13382: F, t13389: F, t13391: F, t13394: F, t13399: F, t13403: F, t13406: F, t13409: F, t2812: F, t2836: F, t3046: F, t4981: F, t9574: F, t9576: F, t9581: F, t9600: F, t979: F) -> (F, F) {
    let t13412 = t13410 * t13411;
    let t13413 = t3200 * t13412;
    let t13415 = F::cast_from(0.33163888888888888888e-2_f64) * t9574 + F::cast_from(0.88437037037037037034e-2_f64) * t9576 - F::cast_from(0.16581944444444444444e-2_f64) * t9581 + F::cast_from(0.16581944444444444444e-2_f64) * t13367 - F::cast_from(0.24872916666666666666e-2_f64) * t13370 - F::cast_from(0.49745833333333333332e-2_f64) * t13373 - F::cast_from(0.22109259259259259258e-2_f64) * t9600 + F::cast_from(0.13345e0_f64) * t979 * t13377 + F::cast_from(0.178089025e-1_f64) * t2836 * t13377 - F::cast_from(0.58958024691358024689e-2_f64) * t13382 + F::cast_from(0.66725e-1_f64) * t4981 * t2812 - F::cast_from(0.66725e-1_f64) * t4981 * t3046 - F::cast_from(0.33163888888888888888e-2_f64) * t13389 + F::cast_from(0.22109259259259259258e-2_f64) * t13391 - F::cast_from(0.88437037037037037034e-2_f64) * t13394 - F::cast_from(0.55273148148148148147e-2_f64) * t13399 - F::cast_from(0.17687407407407407407e-1_f64) * t13403 - F::cast_from(0.88437037037037037034e-2_f64) * t13406 + t13409 - F::cast_from(0.44218518518518518517e-2_f64) * t13413;
    (t13413, t13415)
}
