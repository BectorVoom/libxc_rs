//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 420/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk420(t1222: f64, t1744: f64, t1653: f64, t248: f64, t3521: f64, t1227: f64, t1735: f64, t3570: f64, t1213: f64, t1009: f64, t1720: f64, t1011: f64) -> (f64, f64, f64, f64, f64) {
    let t4959 = t1744 * t1222;
    let t4993 = t248 * t3521 * t1653;
    let t4994 = t1227 * t4993;
    let t4997 = t248 * t3570 * t1735;
    let t4998 = t1213 * t4997;
    let t5000 = t1720 * t1009;
    let t5001 = t5000 * t1011;
    (t4959, t4994, t4998, t5000, t5001)
}
