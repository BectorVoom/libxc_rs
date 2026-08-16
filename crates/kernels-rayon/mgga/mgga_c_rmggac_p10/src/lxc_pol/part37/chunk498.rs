//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 498/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk498(t14019: f64, t14022: f64, t14027: f64, t217: f64, t3127: f64, t3131: f64, t3119: f64, t128: f64, t446: f64, t118: f64, t13862: f64, t3129: f64, t4441: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14028 = t14019 * t14022 * t14027;
    let t14030 = t217 * t3127;
    let t14031 = t14030 * t3131;
    let t14032 = t14031 * t3119;
    let t14033 = t128 * t446;
    let t14034 = t118 * t14033;
    let t14035 = t13862 * t14034;
    let t14036 = t14032 * t14035;
    let t14039 = 1.0_f64 / t3129 / t4441;
    (t14028, t14030, t14031, t14032, t14034, t14035, t14036, t14039)
}
