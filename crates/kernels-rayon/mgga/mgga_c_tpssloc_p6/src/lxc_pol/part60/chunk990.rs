//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 990/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk990(t3941: f64, t5493: f64, t8326: f64, t1458: f64, t1851: f64, t576: f64, t33191: f64, t8657: f64, t33185: f64, t33656: f64, t33659: f64, t24465: f64, t28896: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t127627 = 27.0_f64 * t3941 * t8326 * t5493;
    let t127630 = t1851 * t1458;
    let t127643 = t576 * t5493;
    let t127646 = 27.0_f64 * t33191;
    let t127669 = 27.0_f64 * t127643 * t8657;
    let t127671 = 54.0_f64 * t33185 * t33656;
    let t127673 = 54.0_f64 * t33185 * t33659;
    let t127677 = 54.0_f64 * t24465 * t28896;
    (t127627, t127630, t127646, t127669, t127671, t127673, t127677)
}
