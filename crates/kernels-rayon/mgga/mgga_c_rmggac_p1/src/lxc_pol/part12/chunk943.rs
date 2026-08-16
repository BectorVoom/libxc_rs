//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 943/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk943(t2010: f64, t2415: f64, t4018: f64, t8342: f64, t938: f64, t333: f64, t511: f64, t7230: f64, t7231: f64, t8666: f64, t352: f64, t515: f64) -> (f64, f64, f64, f64) {
    let t40214 = t2010 * t2415 * t4018;
    let t40217 = t2010 * t8342 * t938;
    let t40222 = t7230 * t7231 * t511 * t8666 * t333;
    let t40227 = t7230 * t7231 * t515 * t8666 * t352;
    (t40214, t40217, t40222, t40227)
}
