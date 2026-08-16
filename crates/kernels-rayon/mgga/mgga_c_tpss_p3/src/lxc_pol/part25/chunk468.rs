//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 468/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk468(t1693: f64, t1695: f64, t510: f64, t220: f64, t523: f64, t64: f64, t529: f64, t1705: f64, t532: f64) -> (f64, f64, f64, f64) {
    let t1762 = t1693 * t510 * t1695;
    let t1765 = t220 * t523 * t64;
    let t1766 = t1765 * t529;
    let t1771 = t1705 * t532;
    (t1762, t1765, t1766, t1771)
}
