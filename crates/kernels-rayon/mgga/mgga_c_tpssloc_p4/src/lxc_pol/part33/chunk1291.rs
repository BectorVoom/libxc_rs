//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1291/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1291(t23384: f64, t28481: f64, t28691: f64, t28705: f64, t82431: f64, t28681: f64, t1054: f64, t5943: f64, t1921: f64, t5914: f64, t6688: f64, t225: f64, t28505: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t99151 = t23384 * t28481;
    let t99184 = t23384 * t28691;
    let t99190 = t82431 * t28705;
    let t99205 = t23384 * t28681;
    let t99209 = t1054 * t5943;
    let t99210 = t1921 * t99209;
    let t99214 = t6688 * t5914;
    let t99221 = t28505 * t225;
    (t99151, t99184, t99190, t99205, t99210, t99214, t99221)
}
