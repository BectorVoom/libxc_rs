//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 502/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk502(t4232: f64, t4252: f64, t4255: f64, t4259: f64, t4290: f64, t4351: f64, t5407: f64, t5409: f64, t5417: f64, t5418: f64, t5420: f64, t5421: f64, t5426: f64, t5427: f64, t5429: f64, t5433: f64) -> f64 {
    let t5458 = t5407 - t5409 + t5417 + t5418 + t5420 - t5421 + t4232 + t4252 - t4255 - t4259 + t5426 - t4351 + t5427 + t5429 + t4290 - t5433;
    t5458
}
