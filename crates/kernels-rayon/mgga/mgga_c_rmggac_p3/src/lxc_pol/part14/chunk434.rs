//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 434/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk434(t60: f64, t62: f64, t1173: f64, t1175: f64, t1240: f64, t461: f64, t1171: f64, t225: f64, t226: f64) -> (f64, f64, f64, f64) {
    let t4406 = t60 * t60;
    let t4408 = 1.0_f64 / t62 / t4406;
    let t4435 = t1173 * t1175;
    let t4438 = t461 * t1240;
    let t4441 = t1171 * t225;
    let t4443 = 1.0_f64 / t226 / t4441;
    (t4408, t4435, t4438, t4443)
}
