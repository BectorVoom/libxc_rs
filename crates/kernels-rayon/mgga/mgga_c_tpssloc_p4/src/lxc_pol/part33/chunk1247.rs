//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1247/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1247(t23145: f64, t4166: f64, t25132: f64, t81876: f64, t23047: f64, t1512: f64, t81807: f64, t23040: f64, t1496: f64, t81942: f64, t7497: f64, t81933: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t87199 = t4166 * t23145;
    let t87213 = t81876 * t25132;
    let t87218 = t4166 * t23047;
    let t87243 = t81807 * t1512;
    let t87261 = t4166 * t23040;
    let t87304 = t81942 * t1496;
    let t87306 = t81933 * t7497;
    (t87199, t87213, t87218, t87243, t87261, t87304, t87306)
}
