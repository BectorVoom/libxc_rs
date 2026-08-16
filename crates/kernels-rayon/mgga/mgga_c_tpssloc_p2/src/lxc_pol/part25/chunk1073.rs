//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1073/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1073(t12461: f64, t2094: f64, t193: f64, t200: f64, t2056: f64, t10109: f64, t2053: f64, t2061: f64, t2035: f64, t671: f64, t12020: f64, t2091: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26558 = t2094 * t12461;
    let t26563 = t193 * t200 * t2056;
    let t26728 = t10109 * t2053;
    let t26756 = t193 * t2061;
    let t26977 = t2035 * t671;
    let t26989 = t12020 * t2091;
    (t26558, t26563, t26728, t26756, t26977, t26989)
}
