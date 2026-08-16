//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1130/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1130(t1499: f64, t1898: f64, t249: f64, t1512: f64, t6614: f64, t1516: f64, t6621: f64, t1484: f64, t6638: f64, t6637: f64, t6552: f64, t232: f64, t4282: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7503 = t1499 * t1898;
    let t7504 = t7503 * t249;
    let t7506 = t6614 * t1512;
    let t7508 = t6621 * t1516;
    let t7520 = t6638 * t1484;
    let t7521 = t6637 * t7520;
    let t7522 = t6552 * t7521;
    let t7524 = t4282 * t232;
    (t7503, t7504, t7506, t7508, t7520, t7521, t7522, t7524)
}
