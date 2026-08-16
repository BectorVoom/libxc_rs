//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 306/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk306(t2474: f64, t82: f64, t534: f64, t702: f64, t128: f64, t797: f64, t27: f64, t321: f64, t333: f64, t352: f64, t22: f64, t29: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2475 = t82 * t2474;
    let t2479 = t534 * t702;
    let t2500 = t797 * t128;
    let t2518 = t27 * t321;
    let t2523 = t27 * t333;
    let t2529 = t27 * t352;
    let t2564 = t29 * t22;
    (t2475, t2479, t2500, t2518, t2523, t2529, t2564)
}
