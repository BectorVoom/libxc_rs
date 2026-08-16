//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1046/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1046(t37038: f64, t1266: f64, t818: f64, t826: f64, t11056: f64, t1271: f64, t6100: f64, t819: f64, t1276: f64, t3416: f64, t6767: f64, t1096: f64, t19327: f64, param_eta: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37039 = 154.0_f64 / 27.0_f64 * t37038;
    let t37040 = t1266 * t818;
    let t37041 = t37040 * t826;
    let t37066 = t1271 * t11056;
    let t37074 = param_eta * t6100;
    let t37075 = t819 * t37074;
    let t37076 = 154.0_f64 / 27.0_f64 * t37075;
    let t37078 = t1276 * t11056 * t826;
    let t37204 = t6767 * t3416;
    let t37209 = t19327 * t1096;
    (t37039, t37040, t37041, t37066, t37076, t37078, t37204, t37209)
}
