//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 262/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk262(t1163: f64, t1224: f64, t1225: f64, t1223: f64, t357: f64, t346: f64, t347: f64, t1222: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1227 = t1224 * t1225 * t1163;
    let t1229 = -t1223 - 0.17808333333333333333e-1_f64 * t1227;
    let t1232 = t357 * t357;
    let t1233 = 1.0_f64 / t1232;
    let t1234 = t346 * t1233;
    let t1235 = 1.0_f64 / t347;
    let t1237 = -t1222 / 3.0_f64 - t1227 / 3.0_f64;
    (t1227, t1229, t1232, t1233, t1234, t1235, t1237)
}
