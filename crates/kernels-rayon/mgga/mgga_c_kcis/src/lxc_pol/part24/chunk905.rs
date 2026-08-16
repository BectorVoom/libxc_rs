//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 905/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk905(t19380: f64, t19539: f64, t304: f64, t355: f64, t360: f64, t303: f64, t13238: f64, t13243: f64, t19151: f64, t19153: f64, t19155: f64, t19157: f64, t19162: f64, t19295: f64, t19301: f64, t19304: f64, t19307: f64, t19311: f64, t19313: f64, t19315: f64, t19319: f64, t19322: f64, t3049: f64, t6302: f64, t6545: f64, t979: f64) -> (f64, f64, f64, f64) {
    let t19540 = t19380 + t19539;
    let t19541 = t304 * t19540;
    let t19542 = t19541 * t355;
    let t19543 = t19542 * t360;
    let t19544 = t303 * t19543;
    let t19549 = 0.44218518518518518516e-2_f64 * t19151 - 0.33163888888888888888e-2_f64 * t19153 - 0.33163888888888888888e-2_f64 * t19155 + 0.22109259259259259259e-2_f64 * t19157 - 0.27636574074074074073e-2_f64 * t19162 - 0.66725e-1_f64 * t979 * t19295 - 0.66725e-1_f64 * t3049 * t6545 - 0.33163888888888888888e-2_f64 * t19301 - 0.49745833333333333332e-2_f64 * t19304 - 0.24872916666666666666e-2_f64 * t19307 - 0.13265555555555555555e-1_f64 * t19311 + 0.11054629629629629629e-2_f64 * t19313 - 0.36848765432098765431e-3_f64 * t19315 + 0.14739506172839506173e-2_f64 * t19319 - 0.55273148148148148147e-3_f64 * t19322 + 0.24872916666666666666e-2_f64 * t19544 - 0.88437037037037037035e-2_f64 * t13238 + 0.66725e-1_f64 * t3049 * t6302 + t13243;
    (t19540, t19541, t19544, t19549)
}
