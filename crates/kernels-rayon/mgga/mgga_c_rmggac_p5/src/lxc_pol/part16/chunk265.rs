//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 265/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk265(t1135: f64, t2: f64, t577: f64, t428: f64, t1044: f64, t1050: f64, t1087: f64, t1094: f64, t1104: f64, t1112: f64, t1133: f64, t1140: f64, t1422: f64, t1424: f64, t1429: f64) -> (f64, f64, f64, f64, f64) {
    let t1434 = 0.18311447306006545054e-3_f64 * t1135;
    let t1435 = t577 * t2;
    let t1436 = t1435 * t428;
    let t1437 = 0.18311447306006545054e-3_f64 * t1436;
    let t1438 = -t1422 - t1044 - t1424 + t1429 - t1050 + t1133 - t1094 + t1104 + t1112 - t1087 - t1434 + t1140 - t1437;
    (t1434, t1435, t1436, t1437, t1438)
}
