//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 736/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk736(t252: f64, t9584: f64, t2591: f64, t852: f64, t225: f64, t2711: f64, t2594: f64, t2690: f64, t841: f64, t812: f64, t849: f64, t2697: f64, t2707: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9585 = t9584 * t252;
    let t9587 = t2591 * t852;
    let t9590 = t2711 * t225;
    let t9593 = t2594 * t225;
    let t9600 = t841 * t2690;
    let t9601 = t812 * t9600;
    let t9602 = t9601 * t849;
    let t9604 = t2697 * t2707;
    (t9585, t9587, t9590, t9593, t9602, t9604)
}
