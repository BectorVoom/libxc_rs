//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 429/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk429(t402: f64, t4305: f64, t4052: f64, t417: f64, t171: f64, t4058: f64, t1041: f64, t4151: f64, t418: f64, t971: f64, t377: f64, t4209: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4306 = t4305 * t402;
    let t4309 = t4052 * t417;
    let t4312 = t171 * t4058;
    let t4313 = t4052 * t1041;
    let t4316 = t4151 * t417;
    let t4319 = t418 * t971;
    let t4322 = t4209 * t377;
    (t4306, t4309, t4312, t4313, t4316, t4319, t4322)
}
