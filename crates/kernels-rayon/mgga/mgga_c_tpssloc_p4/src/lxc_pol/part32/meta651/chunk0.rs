//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2077/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2077(t91179: f64, t16060: f64, t6951: f64, t1878: f64, t80730: f64, t6604: f64, t80893: f64, t6925: f64, t6976: f64, t26271: f64, t80779: f64, t22844: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t91180 = 0.11869590291677274911e0_f64 * t91179;
    let t91191 = t16060 * t6951;
    let t91194 = t1878 * t80730;
    let t91198 = t80893 * t6604;
    let t91202 = t6925 * t6976;
    let t91206 = t80779 * t26271;
    let t91208 = t22844 * t6976;
    (t91180, t91191, t91194, t91198, t91202, t91206, t91208)
}
