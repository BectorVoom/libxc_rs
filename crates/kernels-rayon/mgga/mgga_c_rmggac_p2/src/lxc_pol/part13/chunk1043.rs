//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1043/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1043(t38837: f64, t38853: f64, t38857: f64, t38860: f64, t38863: f64, t38869: f64, t34799: f64, t37221: f64, t37222: f64, t37223: f64, t38822: f64, t38826: f64, t38833: f64, t38841: f64, t38846: f64, t38850: f64, t38866: f64) -> f64 {
    let t42755 = 0.1440846329149835838e-2_f64 * t38837;
    let t42759 = 0.20496175532535769482e-3_f64 * t38853;
    let t42760 = 0.1440846329149835838e-2_f64 * t38857;
    let t42761 = 0.1440846329149835838e-2_f64 * t38860;
    let t42762 = 0.1440846329149835838e-2_f64 * t38863;
    let t42764 = 0.20496175532535769482e-3_f64 * t38869;
    let t42765 = -0.20496175532535769482e-3_f64 * t38822 + 0.60975299583150056624e-3_f64 * t38826 - t37221 + t37222 - t37223 - 0.2881692658299671676e-2_f64 * t34799 + 0.60975299583150056624e-3_f64 * t38833 + t42755 - 0.86737941314158990616e-4_f64 * t38841 - 0.86737941314158990616e-4_f64 * t38846 - 0.1440846329149835838e-2_f64 * t38850 - t42759 + t42760 + t42761 + t42762 + 0.72042316457491791901e-3_f64 * t38866 - t42764;
    t42765
}
