//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 720/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk720(t70211: f64, t1986: f64, t2090: f64, t13957: f64, t36292: f64, t739: f64, t14012: f64, t14371: f64, t1341: f64, t638: f64, t669: f64, t7310: f64) -> (f64, f64, f64, f64, f64) {
    let t70212 = 0.10909864661698136691e0_f64 * t70211;
    let t70221 = t1986 * t2090;
    let t70225 = t739 * t36292 * t13957;
    let t70229 = t14371 * t14012;
    let t70230 = 0.1226351426503095703e-4_f64 * t70229;
    let t70237 = t638 * t7310 * t669 * t1341;
    (t70212, t70221, t70225, t70230, t70237)
}
