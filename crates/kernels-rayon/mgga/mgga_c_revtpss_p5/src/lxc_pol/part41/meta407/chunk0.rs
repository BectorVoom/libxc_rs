//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1422/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1422(t30: f64, t1450: f64, t6922: f64, t6785: f64, t9605: f64, t3874: f64, t5824: f64, t1344: f64, t18280: f64, t2255: f64, t5574: f64, t605: f64, t6792: f64, t9617: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t21937 = t6922 * t1450;
    let t21944 = t9605 * t6785;
    let t21949 = t3874 * t5824;
    let t21955 = piecewise3(t31, 0.0_f64, 8.0_f64 / 27.0_f64 * t21944 * t605 - 8.0_f64 / 9.0_f64 * t5574 * t2255 - 2.0_f64 / 9.0_f64 * t21949 * t605 + 2.0_f64 / 3.0_f64 * t1344 * t18280);
    let t21956 = t9617 * t6792;
    (t21937, t21955, t21956)
}
