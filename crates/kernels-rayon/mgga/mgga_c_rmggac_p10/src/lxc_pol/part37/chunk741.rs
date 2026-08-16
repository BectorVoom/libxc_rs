//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 741/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk741(t69936: f64, t69938: f64, t69940: f64, t69942: f64, t14696: f64, t7335: f64, t2019: f64, t3180: f64, t7926: f64, t14572: f64, t7487: f64, t14559: f64, t2020: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t71545 = 0.32526727992809621482e-4_f64 * t69936;
    let t71546 = 0.60975299583150056624e-3_f64 * t69938;
    let t71551 = 0.16263363996404810741e-4_f64 * t69940;
    let t71552 = 0.16263363996404810741e-4_f64 * t69942;
    let t71564 = t7335 * t14696;
    let t71565 = 0.15243824895787514157e-3_f64 * t71564;
    let t71581 = t2019 * t7926 * t3180;
    let t71582 = 0.81300399444200075504e-3_f64 * t71581;
    let t71583 = t7487 * t14572;
    let t71594 = t2019 * t2020 * t14559;
    (t71545, t71546, t71551, t71552, t71565, t71582, t71583, t71594)
}
