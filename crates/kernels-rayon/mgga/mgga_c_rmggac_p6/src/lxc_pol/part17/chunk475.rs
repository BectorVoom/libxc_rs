//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 475/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk475(t4118: f64, t4056: f64, t4062: f64, t4064: f64, t4074: f64, t4077: f64, t4080: f64, t4083: f64, t4089: f64, t4101: f64, t4106: f64, t4111: f64, t5375: f64, t5376: f64, t5971: f64, t5977: f64, t5978: f64) -> (f64, f64) {
    let t5981 = 12.0_f64 * t4118;
    let t5982 = -t4056 + t4062 + t4064 + t5375 - t5376 - t4074 - t5971 - t4077 - t4080 + t4083 + t5977 + t4089 - t4101 + t4106 + t4111 + t5978;
    (t5981, t5982)
}
