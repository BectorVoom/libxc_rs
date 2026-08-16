//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 458/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk458(t2689: f64, t810: f64, t21: f64, t65: f64, t64: f64, t159: f64, t222: f64, t794: f64, t798: f64, t234: f64, t2453: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2691 = 0.76220476654346199061e-4_f64 * t2689 * t810;
    let t2698 = 1.0_f64 / t65 / t21;
    let t2699 = t64 * t2698;
    let t2700 = t2699 * t159;
    let t2702 = 35.0_f64 / 432.0_f64 * t2700 * t222;
    let t2703 = t794 * t798;
    let t2710 = t2453 * t234;
    (t2691, t2698, t2699, t2700, t2702, t2703, t2710)
}
