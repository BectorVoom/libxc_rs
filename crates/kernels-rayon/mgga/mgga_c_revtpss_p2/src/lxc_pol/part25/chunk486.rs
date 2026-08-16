//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 486/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk486(t231: f64, t2645: f64, t827: f64, t828: f64, t820: f64, t843: f64, t849: f64, t857: f64, t2430: f64, t855: f64, t212: f64, t27: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2646 = t2645 * t231;
    let t2648 = t827 * t828 * t2646;
    let t2652 = t820 * t849 * t843;
    let t2653 = t2652 * t857;
    let t2656 = t855 * t828 * t2430;
    let t2659 = t27 * t212;
    (t2646, t2648, t2652, t2653, t2656, t2659)
}
