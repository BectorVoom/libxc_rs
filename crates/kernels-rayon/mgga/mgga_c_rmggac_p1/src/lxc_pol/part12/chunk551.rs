//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 551/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk551(t289: f64, t7399: f64, t1990: f64, t2186: f64, t1271: f64, t1986: f64, t675: f64, t4443: f64, t671: f64, t674: f64, t1175: f64, t128: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7400 = t289 * t7399;
    let t7401 = 0.4726e1_f64 * t7400;
    let t7402 = t2186 * t1990;
    let t7404 = t1986 * t1271;
    let t7405 = t675 * t7404;
    let t7406 = 0.85129199786595678796e-5_f64 * t7405;
    let t7407 = t671 * t4443;
    let t7408 = t7407 * t674;
    let t7409 = t128 * t1175;
    (t7401, t7402, t7404, t7406, t7407, t7408, t7409)
}
