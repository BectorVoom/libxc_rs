//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 247/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk247(t2024: f64, t352: f64, t321: f64, t665: f64, t333: f64, t645: f64, t1343: f64, t36: f64, t71: f64) -> (f64, f64, f64, f64, f64) {
    let t2025 = t2024 * t352;
    let t2028 = t665 * t321;
    let t2031 = t665 * t333;
    let t2034 = t645 * t321;
    let t2038 = t36 * t1343;
    let t2039 = t2038 * t71;
    (t2025, t2028, t2031, t2034, t2039)
}
