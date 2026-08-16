//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 520/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk520(t14174: f64, t1550: f64, t495: f64, t664: f64, t515: f64, t1971: f64, t7230: f64, t498: f64, t7231: f64, t3351: f64, t7799: f64, t3352: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14175 = t1550 * t14174;
    let t14178 = t664 * t495;
    let t14179 = t515 * t14178;
    let t14180 = t1971 * t14179;
    let t14181 = t7230 * t14180;
    let t14182 = 0.1064114997332445985e-4_f64 * t14181;
    let t14183 = t664 * t498;
    let t14184 = t515 * t14183;
    let t14185 = t7231 * t14184;
    let t14186 = t3351 * t14185;
    let t14188 = t515 * t7799;
    let t14189 = t3352 * t14188;
    (t14175, t14180, t14182, t14185, t14186, t14189)
}
