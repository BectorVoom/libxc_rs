//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 567/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk567(t14672: f64, t3351: f64, t14211: f64, t14214: f64, t209: f64, t476: f64, t698: f64, t515: f64, t1971: f64, t1970: f64, t14469: f64, t739: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14673 = t3351 * t14672;
    let t14674 = 0.12769379967989351819e-4_f64 * t14673;
    let t14676 = 0.16263363996404810741e-4_f64 * t14211;
    let t14677 = 0.16263363996404810741e-4_f64 * t14214;
    let t14681 = t698 * t476 * t209;
    let t14682 = t515 * t14681;
    let t14683 = t1971 * t14682;
    let t14684 = t1970 * t14683;
    let t14685 = 0.42564599893297839398e-5_f64 * t14684;
    let t14686 = t739 * t14469;
    (t14674, t14676, t14677, t14683, t14685, t14686)
}
