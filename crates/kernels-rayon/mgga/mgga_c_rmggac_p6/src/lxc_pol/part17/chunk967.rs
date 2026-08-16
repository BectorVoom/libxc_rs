//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 967/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk967(t2289: f64, t38638: f64, t1356: f64, t1923: f64, t2131: f64, t35566: f64, t40085: f64, t40087: f64, t40089: f64, t45994: f64, t45999: f64, t46001: f64, t46003: f64, t46005: f64, t46018: f64, t46020: f64, t46022: f64, t46024: f64, t4985: f64, t5879: f64, t6355: f64, t7399: f64, t7703: f64, t8371: f64, t8399: f64) -> f64 {
    let t46026 = t38638 * t2289;
    let t46028 = -0.23948483403727617128e0_f64 * t6355 * t8371 - t35566 + 0.85129199786595678796e-5_f64 * t45994 - 0.23942587439980034662e-4_f64 * t45999 + t40085 + t40087 + t40089 + 0.44903406381989282115e-1_f64 * t46001 - 0.17961362552795712846e0_f64 * t46003 - 0.11974241701863808564e0_f64 * t1356 * t7703 * t46005 - 0.23948483403727617128e0_f64 * t4985 * t8399 - 0.2363e1_f64 * t5879 * t2131 - 0.2363e1_f64 * t1923 * t7399 - 0.53205749866622299248e-5_f64 * t46018 - 0.25538759935978703638e-4_f64 * t46020 - 0.25538759935978703638e-4_f64 * t46022 + 0.1064114997332445985e-4_f64 * t46024 + 0.59590439850616975155e-4_f64 * t46026;
    t46028
}
