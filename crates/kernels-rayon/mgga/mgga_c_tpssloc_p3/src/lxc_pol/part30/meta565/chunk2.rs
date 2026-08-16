//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1933/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1933(t25: f64, t5664: f64, t1408: f64, t1530: f64, t5660: f64, t1877: f64, t1915: f64, t22959: f64, t23295: f64, t2522: f64, t25358: f64, t28242: f64, t28249: f64, t28252: f64, t28256: f64, t28448: f64, t4314: f64, t5397: f64, t6670: f64, t7475: f64, t7541: f64, t7545: f64) -> (f64, f64, f64, f64) {
    let t28456 = t25 * t5664;
    let t28459 = t1408 * t1530;
    let t28462 = t25 * t5660;
    let t28469 = 3.0_f64 * t4314 * t28242 + 3.0_f64 * t2522 * t7541 * t7475 - 3.0_f64 * t22959 * t28249 + 3.0_f64 * t2522 * t1915 * t28252 + 3.0_f64 / 2.0_f64 * t2522 * t1915 * t28256 + t1877 * t28448 * t25 / 2.0_f64 - t1877 * t25358 * t7545 + t1877 * t7541 * t1408 + t1877 * t23295 * t28456 - t1877 * t6670 * t28459 - t1877 * t6670 * t28462 / 2.0_f64 + t1877 * t1915 * t5397 / 2.0_f64;
    (t28456, t28459, t28462, t28469)
}
