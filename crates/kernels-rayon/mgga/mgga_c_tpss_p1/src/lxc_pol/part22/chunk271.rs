//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 271/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk271(t849: f64, t854: f64, t235: f64, t671: f64, t275: f64, t277: f64, t334: f64) -> (f64, f64, f64, f64, f64) {
    let t855 = t854 * t849;
    let t857 = t671 * t235;
    let t859 = t275 * t857 * t277;
    let t860 = 0.82156666666666666667e-1_f64 * t859;
    let t861 = t235 * t334;
    (t855, t857, t859, t860, t861)
}
