//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1237/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1237(t221: f64, t4433: f64, t10703: f64, t2674: f64, t4353: f64, t9794: f64, t10760: f64, t10890: f64, t1549: f64, t1544: f64, t2430: f64, t2477: f64, t828: f64) -> (f64, f64, f64, f64) {
    let t14756 = t221 * t4433;
    let t14757 = t10703 * t14756;
    let t14759 = 0.50820002809285328225e-3_f64 * t2674 * t14757;
    let t14760 = t9794 * t4353;
    let t14761 = t10760 * t14760;
    let t14765 = t10890 * t1549;
    let t14767 = t1544 * t2430;
    let t14769 = t2477 * t828 * t14767;
    (t14759, t14761, t14765, t14769)
}
