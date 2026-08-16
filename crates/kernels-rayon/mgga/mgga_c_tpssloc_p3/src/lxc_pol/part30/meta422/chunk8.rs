//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1629/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1629(t1437: f64, t4021: f64, t5445: f64, t645: f64, t1409: f64, t65: f64, t67: f64, t1864: f64, t3966: f64, t5392: f64, t628: f64, t17635: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19313 = t1437 * t4021;
    let t19318 = t5445 * t645;
    let t19322 = t1409 * t65 * t67;
    let t19323 = t1864 * t3966;
    let t19326 = t5392 * t628;
    let t19331 = t17635 * t65;
    (t19313, t19318, t19322, t19323, t19326, t19331)
}
