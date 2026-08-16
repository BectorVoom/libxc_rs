//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 471/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk471(t1597: f64, t340: f64, t343: f64, t974: f64, t1593: f64, t971: f64, t973: f64) -> (f64, f64, f64) {
    let t1598 = t340 * t1597;
    let t1599 = t1598 * t343;
    let t1600 = t974 * t1599;
    let t1603 = t971 + 0.27777777777777777777e-3_f64 * t973 * t1593 - 0.83333333333333333332e-3_f64 * t973 * t1600;
    (t1599, t1600, t1603)
}
