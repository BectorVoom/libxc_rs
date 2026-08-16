//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 861/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk861(t2040: f64, t31537: f64, t6534: f64, t89: f64, t7050: f64, t8526: f64, t6535: f64, t7042: f64, t1377: f64, t2091: f64, t1307: f64, t22635: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31539 = 2.0_f64 * t31537 * t2040;
    let t31540 = t89 * t6534;
    let t31542 = 2.0_f64 * t31540 * t2040;
    let t31544 = 2.0_f64 * t8526 * t7050;
    let t31548 = 2.0_f64 * t7042 * t6535;
    let t31549 = t1377 * t2091;
    let t31550 = t31549 * t1307;
    let t31551 = t22635 * t31550;
    (t31539, t31540, t31542, t31544, t31548, t31549, t31550, t31551)
}
