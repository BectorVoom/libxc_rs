//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 770/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk770(t14125: f64, t21708: f64, t9050: f64, t21709: f64, t9054: f64, t14117: f64, t9095: f64, t128: f64, t597: f64, t118: f64, t13862: f64, t14018: f64, t3119: f64, t3131: f64, t69635: f64) -> (f64, f64, f64, f64, f64) {
    let t73906 = t21708 * t14125 * t9050;
    let t73909 = t21708 * t21709 * t9054;
    let t73912 = t21708 * t14117 * t9095;
    let t73917 = t128 * t597;
    let t73920 = t14018 * t69635 * t3131 * t3119 * t13862 * t118 * t73917;
    (t73906, t73909, t73912, t73917, t73920)
}
