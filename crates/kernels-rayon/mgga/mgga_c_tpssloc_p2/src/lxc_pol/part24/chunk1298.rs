//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1298/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1298(t2331: f64, t625: f64, t2332: f64, t22470: f64, t2358: f64, t63: f64, t9365: f64, t9366: f64, t666: f64, t22473: f64, t6530: f64, t9411: f64) -> (f64, f64, f64, f64, f64) {
    let t81442 = t625 * t2331;
    let t81443 = t81442 * t2332;
    let t81445 = t22470 * t2358;
    let t81446 = t63 * t9365;
    let t81447 = t81446 * t9366;
    let t81449 = t666 * t2358;
    let t81450 = t22473 * t81449;
    let t81452 = t6530 * t9411;
    (t81443, t81445, t81447, t81450, t81452)
}
