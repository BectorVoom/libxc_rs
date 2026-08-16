//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 781/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk781(t6712: f64, t995: f64, t1941: f64, t3077: f64, t1942: f64, t3082: f64, t344: f64, t40: f64, t1009: f64, t6740: f64, t1015: f64, t6746: f64) -> (f64, f64, f64, f64, f64) {
    let t23463 = t6712 * t995;
    let t23465 = t3077 * t1941;
    let t23469 = t1942 * t3082 / 6912.0_f64;
    let t23470 = t40 * t344;
    let t23471 = t23470 * t1009;
    let t23472 = t6740 * t23471;
    let t23473 = t1015 * t6746;
    (t23463, t23465, t23469, t23472, t23473)
}
