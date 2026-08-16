//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1001/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1001(t14722: f64, t14704: f64, t1667: f64, t2403: f64, t14720: f64, t4775: f64, t699: f64, t4772: f64, t1657: f64, t3263: f64, t1098: f64, t4737: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14723 = 4.0_f64 / 9.0_f64 * t14722;
    let t14724 = 2.0_f64 / 9.0_f64 * t14704;
    let t14766 = t2403 * t1667;
    let t14768 = 0.13418888888888888889e0_f64 * t14720;
    let t14781 = t699 * t4775;
    let t14782 = 0.22076e0_f64 * t14781;
    let t14818 = t699 * t4772;
    let t14838 = t1657 * t3263;
    let t14845 = t4737 * t1098;
    (t14723, t14724, t14766, t14768, t14781, t14782, t14818, t14838, t14845)
}
