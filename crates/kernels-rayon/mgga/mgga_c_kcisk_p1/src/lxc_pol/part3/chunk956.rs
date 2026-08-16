//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 956/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk956(t14115: f64, t14149: f64, t416: f64, t467: f64, t471: f64, t415: f64, t3508: f64, t3733: f64, t1411: f64, t1220: f64, t13441: f64, t13466: f64, t13470: f64, t13935: f64, t13940: f64, t13947: f64, t13952: f64, t13956: f64, t13960: f64, t13962: f64) -> (f64, f64, f64, f64) {
    let t14150 = t14115 + t14149;
    let t14151 = t416 * t14150;
    let t14152 = t14151 * t467;
    let t14153 = t14152 * t471;
    let t14154 = t415 * t14153;
    let t14156 = t3508 * t3733;
    let t14157 = t1411 * t14156;
    let t14159 = 0.82909722222222222219e-2_f64 * t13466 - 0.8290972222222222222e-2_f64 * t13470 - 0.193e0_f64 * t1220 * t13935 + 0.2653111111111111111e-1_f64 * t13940 - 0.386e0_f64 * t1220 * t13441 + 0.1492375e-1_f64 * t13947 - 0.49745833333333333332e-2_f64 * t13952 - 0.11054629629629629629e-2_f64 * t13956 + 0.44218518518518518516e-2_f64 * t13960 + 0.49745833333333333332e-2_f64 * t13962 + 0.24872916666666666666e-2_f64 * t14154 - 0.74618749999999999998e-2_f64 * t14157;
    (t14150, t14154, t14157, t14159)
}
