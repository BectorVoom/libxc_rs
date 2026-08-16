//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 710/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk710(t8694: f64, t8696: f64, t8698: f64, t9499: f64, t9040: f64, t9060: f64, t9062: f64, t9075: f64, t9079: f64, t9083: f64, t9091: f64, t9650: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10358 = 0.11918087970123395032e-3_f64 * t8694;
    let t10359 = 0.11918087970123395032e-3_f64 * t8696;
    let t10360 = 0.39726959900411316772e-4_f64 * t8698;
    let t10376 = 2.0_f64 * t9499;
    let t10384 = 0.39726959900411316772e-4_f64 * t9040;
    let t10385 = 0.47896966807455234256e0_f64 * t9060;
    let t10386 = 0.3193131120497015617e0_f64 * t9062;
    let t10487 = 0.15965655602485078085e0_f64 * t9075;
    let t10496 = 0.15965655602485078085e0_f64 * t9079;
    let t10503 = 0.1440846329149835838e-2_f64 * t9083;
    let t10504 = 0.39726959900411316772e-4_f64 * t9091;
    let t10508 = 2.0_f64 * t9650;
    (t10358, t10359, t10360, t10376, t10384, t10385, t10386, t10487, t10496, t10503, t10504, t10508)
}
