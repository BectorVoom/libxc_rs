//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 832/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk832(t1652: f64, t3351: f64, t498: f64, t515: f64, t7231: f64, t29892: f64, t3352: f64, t2010: f64, t2012: f64, t5061: f64, t4601: f64, t8551: f64) -> (f64, f64, f64, f64) {
    let t38724 = t3351 * t7231 * t515 * t1652 * t498;
    let t38728 = t3351 * t3352 * t515 * t29892;
    let t38733 = t2010 * t2012 * t5061;
    let t38739 = t4601 * t8551;
    (t38724, t38728, t38733, t38739)
}
