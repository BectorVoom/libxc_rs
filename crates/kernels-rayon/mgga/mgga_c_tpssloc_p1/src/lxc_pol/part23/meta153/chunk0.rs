//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 711/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk711(t3441: f64, t5392: f64, t3440: f64, t4904: f64, t4919: f64, t3455: f64, t1177: f64, t1178: f64, t5398: f64, t3464: f64, t4770: f64, t6012: f64, t6015: f64, t6018: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6119 = t3441 * t5392;
    let t6120 = t3440 * t6119;
    let t6123 = t4919 * t4904;
    let t6126 = t3455 * t5392;
    let t6127 = t1177 * t6126;
    let t6130 = t1178 * t5398;
    let t6131 = t1177 * t6130;
    let t6138 = -t3464 + 2.0_f64 / 9.0_f64 * t4770 + t6012 / 18.0_f64 - t6015 / 3.0_f64 - t6018 / 6.0_f64;
    (t6119, t6120, t6123, t6126, t6127, t6130, t6131, t6138)
}
