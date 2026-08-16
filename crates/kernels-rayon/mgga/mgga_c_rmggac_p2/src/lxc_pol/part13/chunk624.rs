//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 624/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk624(t8216: f64, t7903: f64, t7906: f64, t7912: f64, t7915: f64, t333: f64, t698: f64, t352: f64, t321: f64, t7818: f64, t7820: f64, t338: f64, t8159: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8217 = 0.23948483403727617128e0_f64 * t8216;
    let t8219 = 0.5107751987195740728e-4_f64 * t7903;
    let t8220 = 0.2553875993597870364e-4_f64 * t7906;
    let t8223 = 0.1702583995731913576e-4_f64 * t7912;
    let t8224 = 0.10215503974391481456e-3_f64 * t7915;
    let t8231 = t698 * t333;
    let t8232 = t8231 * t352;
    let t8235 = t698 * t321;
    let t8236 = t8235 * t352;
    let t8242 = 0.2927036860455597649e0_f64 * t7818;
    let t8243 = 0.66671395154821946452e-1_f64 * t7820;
    let t8244 = t338 * t8159;
    (t8217, t8219, t8220, t8223, t8224, t8232, t8235, t8236, t8242, t8243, t8244)
}
