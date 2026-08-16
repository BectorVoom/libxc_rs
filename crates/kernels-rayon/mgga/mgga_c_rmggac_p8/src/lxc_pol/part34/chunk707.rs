//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 707/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk707(t14107: f64, t3807: f64, t13980: f64, t2019: f64, t2020: f64, t13984: f64, t14193: f64, t16156: f64, t13815: f64, t2165: f64, t7553: f64, t217: f64, t3119: f64, t457: f64, t7715: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t69710 = t3807 * t14107;
    let t69722 = t2019 * t2020 * t13980;
    let t69728 = t2019 * t2020 * t13984;
    let t69742 = t16156 * t14193;
    let t69745 = t7553 * t13815 * t2165;
    let t69755 = t217 * t457 * t7715 * t3119;
    (t69710, t69722, t69728, t69742, t69745, t69755)
}
