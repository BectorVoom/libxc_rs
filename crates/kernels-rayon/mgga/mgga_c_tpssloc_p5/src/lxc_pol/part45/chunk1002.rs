//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1002/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1002(t31304: f64, t6880: f64, t1874: f64, t84097: f64, t31537: f64, t7057: f64, t22479: f64, t89: f64, t2040: f64, t31540: f64, t7050: f64, t2314: f64, t31747: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t115245 = 6.0_f64 * t31304 * t6880;
    let t115249 = 2.0_f64 * t84097 * t1874;
    let t115251 = 4.0_f64 * t31537 * t7057;
    let t115252 = t89 * t22479;
    let t115254 = 2.0_f64 * t115252 * t2040;
    let t115256 = 4.0_f64 * t31540 * t7050;
    let t115261 = 4.0_f64 * t2314 * t31747;
    (t115245, t115249, t115251, t115254, t115256, t115261)
}
