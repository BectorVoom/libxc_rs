//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 619/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk619(t118: f64, t1799: f64, t794: f64, t3739: f64, t1808: f64, t225: f64, t1811: f64, t3726: f64, t1814: f64, t68: f64) -> (f64, f64, f64, f64, f64) {
    let t5202 = t118 * t794 * t1799;
    let t5203 = t3739 * t5202;
    let t5215 = t1808 * t225;
    let t5220 = t3726 * t1811;
    let t5234 = t1814 * t68;
    (t5202, t5203, t5215, t5220, t5234)
}
