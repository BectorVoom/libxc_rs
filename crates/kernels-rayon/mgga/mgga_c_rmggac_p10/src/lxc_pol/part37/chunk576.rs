//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 576/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk576(t14935: f64, t338: f64, t118: f64, t14338: f64, t14333: f64, t14335: f64, t14354: f64, t14521: f64, t14523: f64, t14524: f64, t14527: f64, t14969: f64, t14977: f64, t14981: f64, t305: f64, t326: f64) -> (f64, f64, f64, f64) {
    let t15001 = t338 * t14935;
    let t15002 = t118 * t15001;
    let t15007 = 0.16566831523319392754e-1_f64 * t14338;
    let t15012 = -t14333 + t14335 + 0.59871208509319042821e-1_f64 * t305 * t14969 - t14521 + t15007 - 0.59871208509319042821e-1_f64 * t326 * t14977 - 0.39914139006212695214e-1_f64 * t118 * t14981 + t14523 + t14524 + t14354 + t14527;
    (t15001, t15002, t15007, t15012)
}
