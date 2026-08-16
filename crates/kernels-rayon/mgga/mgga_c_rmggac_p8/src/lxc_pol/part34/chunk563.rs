//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 563/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk563(t14530: f64, t82: f64, t72: f64, t13851: f64, t13854: f64, t13856: f64, t13859: f64, t13864: f64, t13869: f64, t13873: f64, t13877: f64, t13881: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14531 = t82 * t14530;
    let t14532 = t72 * t14531;
    let t14533 = 0.1276937996798935182e-4_f64 * t13851;
    let t14534 = 0.58171619854173713846e-5_f64 * t13854;
    let t14535 = 0.85129199786595678799e-5_f64 * t13856;
    let t14536 = 0.85129199786595678799e-5_f64 * t13859;
    let t14537 = 0.17519306092901367188e-6_f64 * t13864;
    let t14538 = 0.15961724959986689775e-4_f64 * t13869;
    let t14539 = 0.1276937996798935182e-4_f64 * t13873;
    let t14540 = 0.2553875993597870364e-4_f64 * t13877;
    let t14541 = 0.3830813990396805546e-4_f64 * t13881;
    (t14531, t14532, t14533, t14534, t14535, t14536, t14537, t14538, t14539, t14540, t14541)
}
