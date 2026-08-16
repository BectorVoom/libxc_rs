//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 869/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk869(t41146: f64, t41160: f64, t41170: f64, t41195: f64, t41297: f64, t41308: f64, t41314: f64, t41319: f64, t41323: f64, t41338: f64, t41347: f64, t41371: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43507 = 0.3193131120497015617e0_f64 * t41146;
    let t43513 = 0.14161231045397953428e-1_f64 * t41160;
    let t43518 = 0.21241846568096930142e-1_f64 * t41170;
    let t43530 = 0.15965655602485078085e0_f64 * t41195;
    let t43588 = 0.24244143692662525982e0_f64 * t41297;
    let t43592 = 0.14546486215597515589e0_f64 * t41308;
    let t43594 = 0.14546486215597515589e0_f64 * t41314;
    let t43596 = 0.4838420607177634088e-2_f64 * t41319;
    let t43598 = 0.67737888500486877232e-2_f64 * t41323;
    let t43606 = 0.31931311204970156172e0_f64 * t41338;
    let t43611 = 0.9676841214355268176e-3_f64 * t41347;
    let t43628 = 0.10643770401656718724e0_f64 * t41371;
    (t43507, t43513, t43518, t43530, t43588, t43592, t43594, t43596, t43598, t43606, t43611, t43628)
}
