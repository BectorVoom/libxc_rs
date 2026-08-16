//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1197/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1197(t112: f64, t12512: f64, t111: f64, t3931: f64, t2311: f64, t671: f64, t2363: f64, t649: f64, t89: f64, t9416: f64, t88: f64, t2745: f64, t776: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45557 = t12512 * t112;
    let t45560 = t3931 * t111;
    let t45602 = t2311 * t671;
    let t45637 = t649 * t2363;
    let t45640 = t89 * t9416;
    let t45814 = t88 * t9416;
    let t46240 = t2745 * t776;
    (t45557, t45560, t45602, t45637, t45640, t45814, t46240)
}
