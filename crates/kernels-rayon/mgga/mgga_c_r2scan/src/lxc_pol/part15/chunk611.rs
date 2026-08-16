//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 611/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk611(t3358: f64, t826: f64, t1070: f64, t1271: f64, t1276: f64, t502: f64, param_eta: f64) -> (f64, f64, f64, f64, f64) {
    let t3359 = t3358 * t826;
    let t3361 = t1271 * t1070;
    let t3363 = t1070 * t826;
    let t3364 = t1276 * t3363;
    let t3366 = param_eta * t502;
    (t3359, t3361, t3363, t3364, t3366)
}
