//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1238/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1238(t1894: f64, t7496: f64, t6591: f64, t1510: f64, t815: f64, t6605: f64, t1499: f64, t1898: f64, t249: f64, t1512: f64, t6614: f64, t1516: f64, t6621: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7497 = t1894 * t7496;
    let t7498 = t6591 * t7497;
    let t7500 = t815 * t1510;
    let t7501 = t6605 * t7500;
    let t7503 = t1499 * t1898;
    let t7504 = t7503 * t249;
    let t7506 = t6614 * t1512;
    let t7508 = t6621 * t1516;
    (t7497, t7498, t7500, t7501, t7503, t7504, t7506, t7508)
}
