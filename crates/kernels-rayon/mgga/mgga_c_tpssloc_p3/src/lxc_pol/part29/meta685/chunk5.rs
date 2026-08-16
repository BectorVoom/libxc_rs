//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2343/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2343(t2109: f64, t90090: f64, t90094: f64, t45844: f64, t7245: f64, t22546: f64, t22549: f64, t24514: f64, t24517: f64, t26016: f64, t7432: f64, t85470: f64, t85473: f64, t85476: f64, t85507: f64, t90072: f64, t90076: f64, t90098: f64, t90101: f64, t90104: f64) -> f64 {
    let t96110 = t2109 * t90090;
    let t96115 = t2109 * t90094;
    let t96120 = t45844 * t7245;
    let t96133 = -5.0_f64 / 3.0_f64 * t26016 * t85476 - 10.0_f64 * t24514 * t90072 - 10.0_f64 / 3.0_f64 * t22549 * t96110 - 10.0_f64 * t24514 * t90076 - 10.0_f64 / 3.0_f64 * t22549 * t96115 - 5.0_f64 / 3.0_f64 * t85507 * t7432 - 5.0_f64 * t96120 * t22546 - 10.0_f64 / 3.0_f64 * t90098 * t24517 - 10.0_f64 / 3.0_f64 * t90101 * t24517 - 10.0_f64 / 3.0_f64 * t90104 * t24517 - 10.0_f64 / 3.0_f64 * t26016 * t85470 - 10.0_f64 / 3.0_f64 * t26016 * t85473;
    t96133
}
