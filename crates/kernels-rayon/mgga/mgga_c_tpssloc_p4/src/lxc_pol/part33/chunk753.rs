//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 753/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk753(t652: f64, t7468: f64, t1458: f64, t1976: f64, t1484: f64, t25: f64, t1915: f64, t6554: f64) -> (f64, f64, f64, f64, f64) {
    let t7470 = 2.0_f64 * t652 * t7468;
    let t7472 = t1976 * t1458;
    let t7475 = t25 * t1484;
    let t7476 = t1915 * t7475;
    let t7479 = t6554 * t1484;
    (t7470, t7472, t7475, t7476, t7479)
}
