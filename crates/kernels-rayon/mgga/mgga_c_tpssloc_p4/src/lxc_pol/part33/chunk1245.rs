//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1245/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1245(t1519: f64, t794: f64, t7480: f64, t81632: f64, t23030: f64, t25035: f64, t23228: f64, t7479: f64, t81573: f64, t23012: f64, t7485: f64, t7489: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t86893 = t794 * t1519;
    let t86903 = t81632 * t7480;
    let t86911 = t23030 * t25035;
    let t86916 = t81573 * t23228 * t7479;
    let t86955 = t23012 * t7485;
    let t86991 = t23012 * t7489;
    (t86893, t86903, t86911, t86916, t86955, t86991)
}
