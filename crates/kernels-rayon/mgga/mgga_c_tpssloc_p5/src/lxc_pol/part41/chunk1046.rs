//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1046/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1046(t16752: f64, t232: f64, t860: f64, t2732: f64, t5612: f64, t1509: f64, t1519: f64, t829: f64, t4234: f64, t4282: f64, t5550: f64, t9573: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16753 = t16752 * t232;
    let t16754 = t860 * t16753;
    let t16756 = t2732 * t5612;
    let t16758 = t1519 * t1509;
    let t16759 = t16758 * t829;
    let t16762 = t4282 * t4234;
    let t16769 = t9573 * t5550;
    (t16753, t16754, t16756, t16758, t16759, t16762, t16769)
}
