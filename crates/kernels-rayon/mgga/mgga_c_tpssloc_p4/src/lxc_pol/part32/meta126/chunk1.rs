//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 727/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk727(t1932: f64, t360: f64, t390: f64, t1878: f64, t268: f64, t405: f64, t1091: f64, t690: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3201 = t1932 * t360;
    let t3215 = t390 * t390;
    let t3216 = 1.0_f64 / t3215;
    let t3236 = t268 * t1878 * t405;
    let t3237 = 0.23744444444444444444e-1_f64 * t3236;
    let t3238 = t690 * t1091;
    (t3201, t3215, t3216, t3236, t3237, t3238)
}
