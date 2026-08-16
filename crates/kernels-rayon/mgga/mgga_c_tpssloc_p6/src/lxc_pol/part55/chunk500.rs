//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 500/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk500(t1932: f64, t3131: f64, t1014: f64, t3032: f64, t3031: f64, t390: f64, t1878: f64, t268: f64, t405: f64, t1091: f64, t690: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3188 = t1932 * t3131;
    let t3199 = t3032 * t1014;
    let t3200 = t3031 * t3199;
    let t3215 = t390 * t390;
    let t3216 = 1.0_f64 / t3215;
    let t3236 = t268 * t1878 * t405;
    let t3237 = 0.23744444444444444444e-1_f64 * t3236;
    let t3238 = t690 * t1091;
    (t3188, t3200, t3216, t3236, t3237, t3238)
}
