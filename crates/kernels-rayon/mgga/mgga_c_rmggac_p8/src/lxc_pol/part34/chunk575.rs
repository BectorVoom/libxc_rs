//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 575/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk575(t14664: f64, t498: f64, t698: f64, t515: f64, t7231: f64, t3351: f64, t8235: f64, t3352: f64, t14211: f64, t14214: f64, t14217: f64, t14220: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14665 = 0.42564599893297839398e-5_f64 * t14664;
    let t14666 = t698 * t498;
    let t14667 = t515 * t14666;
    let t14668 = t7231 * t14667;
    let t14669 = t3351 * t14668;
    let t14670 = 0.42564599893297839398e-5_f64 * t14669;
    let t14671 = t515 * t8235;
    let t14672 = t3352 * t14671;
    let t14673 = t3351 * t14672;
    let t14674 = 0.12769379967989351819e-4_f64 * t14673;
    let t14676 = 0.16263363996404810741e-4_f64 * t14211;
    let t14677 = 0.16263363996404810741e-4_f64 * t14214;
    let t14678 = 0.3252672799280962148e-5_f64 * t14217;
    let t14679 = 0.3252672799280962148e-5_f64 * t14220;
    (t14665, t14668, t14670, t14672, t14674, t14676, t14677, t14678, t14679)
}
