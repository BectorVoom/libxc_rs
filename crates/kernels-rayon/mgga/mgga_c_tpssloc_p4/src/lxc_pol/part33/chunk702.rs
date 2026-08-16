//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 702/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk702(t1891: f64, t6597: f64, t133: f64, t119: f64, t212: f64, t1895: f64, t213: f64, t225: f64) -> (f64, f64, f64, f64, f64) {
    let t6598 = t6597 * t1891;
    let t6599 = t6598 * t133;
    let t6600 = t119 * t212;
    let t6601 = t6600 * t1895;
    let t6602 = t6599 * t6601;
    let t6603 = 0.33643963411783659045e-4_f64 * t6602;
    let t6604 = t213 * t225;
    (t6598, t6600, t6601, t6603, t6604)
}
