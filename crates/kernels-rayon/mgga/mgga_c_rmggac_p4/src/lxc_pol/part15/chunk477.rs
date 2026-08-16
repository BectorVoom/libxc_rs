//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 477/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk477(t1842: f64, t381: f64, t385: f64, t5404: f64, t4342: f64, t5420: f64, t4352: f64, t4214: f64, t4220: f64, t4232: f64, t4252: f64, t4255: f64, t4259: f64, t4338: f64, t4351: f64, t5407: f64, t5409: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5997 = t381 * t1842;
    let t5998 = 4.0_f64 * t5997;
    let t5999 = t385 * t1842;
    let t6000 = 4.0_f64 * t5999;
    let t6001 = 16.0_f64 * t5404;
    let t6002 = 0.11696447245269292414e1_f64 * t4342;
    let t6003 = 2.0_f64 * t5420;
    let t6004 = 0.24415263074675393405e-3_f64 * t4352;
    let t6005 = -t4338 + t5998 - t6000 + t4214 - t4220 - t6001 - t5407 - t5409 + t6002 + t6003 + t4232 + t4252 - t4255 - t4259 - t4351 + t6004;
    (t5998, t6000, t6001, t6002, t6003, t6004, t6005)
}
