//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 977/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk977(t26977: f64, t6535: f64, t22561: f64, t7042: f64, t114422: f64, t26161: f64, t26558: f64, t31304: f64, t6880: f64, t1874: f64, t84097: f64, t31537: f64, t7057: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t115231 = 4.0_f64 * t26977 * t6535;
    let t115233 = 4.0_f64 * t7042 * t22561;
    let t115238 = 4.0_f64 * t26161 * t26558 * t114422;
    let t115245 = 6.0_f64 * t31304 * t6880;
    let t115249 = 2.0_f64 * t84097 * t1874;
    let t115251 = 4.0_f64 * t31537 * t7057;
    (t115231, t115233, t115238, t115245, t115249, t115251)
}
