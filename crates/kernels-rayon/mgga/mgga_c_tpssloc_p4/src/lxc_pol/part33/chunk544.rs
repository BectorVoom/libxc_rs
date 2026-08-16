//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 544/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk544(t1020: f64, t4630: f64, t1009: f64, t1603: f64, t1011: f64, t1019: f64, t1040: f64, t1611: f64, t1626: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4631 = t1020 * t4630;
    let t4639 = t1603 * t1009;
    let t4640 = t4639 * t1011;
    let t4641 = t4640 * t1019;
    let t4644 = t1611 * t1040;
    let t4660 = t1626 * t225;
    (t4631, t4639, t4640, t4641, t4644, t4660)
}
