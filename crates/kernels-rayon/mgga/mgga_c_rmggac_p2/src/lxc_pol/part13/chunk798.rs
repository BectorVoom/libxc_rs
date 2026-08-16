//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 798/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk798(t36268: f64, t7198: f64, t7197: f64, t899: f64, t271: f64, t3899: f64, t638: f64, t641: f64, t36293: f64, t739: f64, t36247: f64, t35979: f64, t4044: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36976 = t7198 * t36268;
    let t36978 = t899 * t7197;
    let t36983 = t638 * t3899 * t271 * t641;
    let t36998 = t739 * t36293;
    let t37000 = t739 * t36247;
    let t37006 = t4044 * t35979;
    (t36976, t36978, t36983, t36998, t37000, t37006)
}
