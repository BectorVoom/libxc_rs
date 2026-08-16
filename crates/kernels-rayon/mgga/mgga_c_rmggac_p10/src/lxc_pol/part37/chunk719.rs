//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 719/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk719(t13839: f64, t2044: f64, t352: f64, t7554: f64, t333: f64, t7273: f64, t14362: f64, t1993: f64, t13868: f64, t13797: f64, t14077: f64, t7282: f64) -> (f64, f64, f64, f64, f64) {
    let t70194 = t13839 * t2044 * t7554 * t352;
    let t70195 = 0.16566831523319392754e-1_f64 * t70194;
    let t70198 = t7273 * t2044 * t7554 * t333;
    let t70207 = t1993 * t14362;
    let t70208 = t70207 * t13868;
    let t70211 = t7282 * t14077 * t13797;
    (t70195, t70198, t70207, t70208, t70211)
}
