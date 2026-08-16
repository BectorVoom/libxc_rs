//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 497/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk497(t4179: f64, t498: f64, t1818: f64, t195: f64, t1835: f64, t500: f64, t1022: f64, t1532: f64, t1819: f64, t1911: f64, t4183: f64, t4214: f64, t4220: f64, t4232: f64, t4252: f64, t4255: f64, t4259: f64, t4336: f64, t4338: f64, t4351: f64, t5407: f64, t5409: f64, t5998: f64, t6000: f64, t6001: f64, t6002: f64, t6003: f64) -> f64 {
    let t6284 = t4179 * t498;
    let t6287 = t195 * t1818;
    let t6290 = t500 * t1835;
    let t6293 = t4336 - t4338 - 0.31091e-1_f64 * t1911 * t1532 + t5998 - t6000 + 0.62182e-1_f64 * t1819 * t6284 - 0.93273e-1_f64 * t6287 * t4183 + t4214 - t4220 - t6001 - t5407 - t5409 + t6002 + t6003 + 0.93273e-1_f64 * t1022 * t6290 + t4232 + t4252 - t4255 - t4259 - t4351;
    t6293
}
