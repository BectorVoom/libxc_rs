//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1132/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1132(t1404: f64, t8496: f64, t111: f64, t31028: f64, t214: f64, t6624: f64, t30657: f64, t6547: f64, t30671: f64, t23030: f64, t30660: f64, t23204: f64, t30656: f64, t6562: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t112518 = t8496 * t1404;
    let t112594 = t31028 * t111;
    let t112660 = t214 * t6624;
    let t112667 = t6547 * t30657;
    let t112673 = t6547 * t30671;
    let t112676 = 0.52089578783527170489e-1_f64 * t23030 * t30660;
    let t112678 = t6562 * t23204 * t30656;
    (t112518, t112594, t112660, t112667, t112673, t112676, t112678)
}
