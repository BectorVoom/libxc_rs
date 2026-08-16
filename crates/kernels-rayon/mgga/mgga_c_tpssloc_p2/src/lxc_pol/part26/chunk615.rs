//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 615/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk615(t1203: f64, t1222: f64, t221: f64, t3426: f64, t456: f64, t1197: f64, t135: f64, t1174: f64, t1196: f64, t2250: f64, t974: f64, t1176: f64, t3247: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3543 = t1203 * t1222;
    let t3545 = t221 * t3426;
    let t3547 = t456 * t3545 / 432.0_f64;
    let t3548 = t135 * t1197;
    let t3549 = t1174 * t3548;
    let t3551 = t1196 * t2250;
    let t3552 = t974 * t3551;
    let t3555 = t1176 * t3247;
    (t3543, t3545, t3547, t3548, t3549, t3551, t3552, t3555)
}
