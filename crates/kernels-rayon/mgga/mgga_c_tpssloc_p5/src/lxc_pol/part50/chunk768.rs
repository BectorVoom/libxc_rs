//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 768/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk768(t1530: f64, t25: f64, t1408: f64, t1877: f64, t1915: f64, t2522: f64, t6670: f64, t7476: f64, t7541: f64, t1539: f64, t6690: f64) -> (f64, f64, f64) {
    let t7545 = t25 * t1530;
    let t7552 = 3.0_f64 / 2.0_f64 * t2522 * t7476 + t1877 * t7541 * t25 / 2.0_f64 - t1877 * t6670 * t7545 / 2.0_f64 + t1877 * t1915 * t1408 / 2.0_f64;
    let t7553 = t6690 * t1539;
    (t7545, t7552, t7553)
}
