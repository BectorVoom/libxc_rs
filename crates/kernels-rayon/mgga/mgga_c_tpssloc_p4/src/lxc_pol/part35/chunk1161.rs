//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1161/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1161(t22804: f64, t7709: f64, t1361: f64, t1799: f64, t22690: f64, t22792: f64, t1878: f64, t22683: f64, t22844: f64, t6604: f64, t22779: f64, t7712: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26268 = t22804 * t7709;
    let t26271 = t22690 * t1361 * t1799;
    let t26272 = t22792 * t26271;
    let t26284 = t1878 * t22683;
    let t26288 = t22844 * t6604;
    let t26295 = t22779 * t7712;
    (t26268, t26271, t26272, t26284, t26288, t26295)
}
