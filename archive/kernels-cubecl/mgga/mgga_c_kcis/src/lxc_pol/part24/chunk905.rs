//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 905/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk905<F: Float>(t19380: F, t19539: F, t304: F, t355: F, t360: F, t303: F, t13238: F, t13243: F, t19151: F, t19153: F, t19155: F, t19157: F, t19162: F, t19295: F, t19301: F, t19304: F, t19307: F, t19311: F, t19313: F, t19315: F, t19319: F, t19322: F, t3049: F, t6302: F, t6545: F, t979: F) -> (F, F, F, F) {
    let t19540 = t19380 + t19539;
    let t19541 = t304 * t19540;
    let t19542 = t19541 * t355;
    let t19543 = t19542 * t360;
    let t19544 = t303 * t19543;
    let t19549 = F::cast_from(0.44218518518518518516e-2_f64) * t19151 - F::cast_from(0.33163888888888888888e-2_f64) * t19153 - F::cast_from(0.33163888888888888888e-2_f64) * t19155 + F::cast_from(0.22109259259259259259e-2_f64) * t19157 - F::cast_from(0.27636574074074074073e-2_f64) * t19162 - F::cast_from(0.66725e-1_f64) * t979 * t19295 - F::cast_from(0.66725e-1_f64) * t3049 * t6545 - F::cast_from(0.33163888888888888888e-2_f64) * t19301 - F::cast_from(0.49745833333333333332e-2_f64) * t19304 - F::cast_from(0.24872916666666666666e-2_f64) * t19307 - F::cast_from(0.13265555555555555555e-1_f64) * t19311 + F::cast_from(0.11054629629629629629e-2_f64) * t19313 - F::cast_from(0.36848765432098765431e-3_f64) * t19315 + F::cast_from(0.14739506172839506173e-2_f64) * t19319 - F::cast_from(0.55273148148148148147e-3_f64) * t19322 + F::cast_from(0.24872916666666666666e-2_f64) * t19544 - F::cast_from(0.88437037037037037035e-2_f64) * t13238 + F::cast_from(0.66725e-1_f64) * t3049 * t6302 + t13243;
    (t19540, t19541, t19544, t19549)
}
