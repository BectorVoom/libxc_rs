//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1061/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1061(t1178: f64, t5398: f64, t1177: f64, t3464: f64, t4770: f64, t6012: f64, t6015: f64, t6018: f64, t457: f64, t460: f64, t974: f64, t1714: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6130 = t1178 * t5398;
    let t6131 = t1177 * t6130;
    let t6138 = -t3464 + 2.0_f64 / 9.0_f64 * t4770 + t6012 / 18.0_f64 - t6015 / 3.0_f64 - t6018 / 6.0_f64;
    let t6139 = t457 * t6138;
    let t6140 = t6139 * t460;
    let t6141 = t974 * t6140;
    let t6144 = t1714 * t1714;
    (t6130, t6131, t6138, t6140, t6141, t6144)
}
