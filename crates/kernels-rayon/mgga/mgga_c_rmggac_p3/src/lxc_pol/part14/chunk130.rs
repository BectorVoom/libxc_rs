//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 130/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk130(t155: f64, t422: f64, t181: f64, t388: f64, t156: f64, t2: f64, t180: f64, t243: f64, t245: f64) -> (f64, f64, f64, f64) {
    let t423 = t155 * t422;
    let t425 = 0.19751673498613801407e-1_f64 * t388 * t181;
    let t426 = t156 * t2;
    let t428 = t243 * t245 * t180;
    (t423, t425, t426, t428)
}
