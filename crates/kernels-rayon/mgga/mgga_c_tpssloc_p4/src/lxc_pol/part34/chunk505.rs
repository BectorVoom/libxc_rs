//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 505/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk505(t1227: f64, t4993: f64, t1735: f64, t248: f64, t3570: f64, t1213: f64, t1009: f64, t1720: f64, t1011: f64, t1212: f64, t1226: f64, t1730: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4994 = t1227 * t4993;
    let t4997 = t248 * t3570 * t1735;
    let t4998 = t1213 * t4997;
    let t5000 = t1720 * t1009;
    let t5001 = t5000 * t1011;
    let t5002 = t5001 * t1212;
    let t5005 = t1730 * t1226;
    (t4994, t4997, t4998, t5000, t5001, t5002, t5005)
}
