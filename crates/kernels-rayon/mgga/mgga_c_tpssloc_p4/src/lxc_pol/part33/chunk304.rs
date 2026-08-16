//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 304/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk304(t1287: f64, t17: f64, t521: f64, t67: f64, t758: f64, t172: f64) -> (f64, f64, f64, f64) {
    let t1288 = t17 * t1287;
    let t1291 = t521 * t67;
    let t1293 = 0.18311447306006545054e-3_f64 * t1291 * t758;
    let t1294 = t521 * t172;
    (t1288, t1291, t1293, t1294)
}
