//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 797/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk797(t36268: f64, t7198: f64, t7197: f64, t899: f64, t271: f64, t3899: f64, t638: f64, t641: f64, t1347: f64, t2128: f64, t7399: f64, t942: f64) -> (f64, f64, f64, f64, f64) {
    let t36976 = t7198 * t36268;
    let t36978 = t899 * t7197;
    let t36983 = t638 * t3899 * t271 * t641;
    let t36984 = 0.69557008413371175709e-2_f64 * t36983;
    let t36992 = t1347 * t2128;
    let t36994 = t942 * t7399;
    (t36976, t36978, t36984, t36992, t36994)
}
