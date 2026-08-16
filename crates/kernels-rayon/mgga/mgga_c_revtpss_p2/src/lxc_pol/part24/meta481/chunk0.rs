//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1470/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1470(t17376: f64, t17524: f64, t17528: f64, t3140: f64, t6564: f64, t3599: f64, t17361: f64, t5274: f64, t1234: f64, t21271: f64, t21093: f64, t372: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t69680 = t17376 * t17524;
    let t69683 = t17376 * t17528;
    let t69692 = t6564 * t3140;
    let t69693 = t69692 * t3599;
    let t69700 = t5274 * t17361;
    let t69795 = t1234 * t21271;
    let t69832 = t372 * t21093;
    (t69680, t69683, t69692, t69693, t69700, t69795, t69832)
}
