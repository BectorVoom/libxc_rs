//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 877/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk877(t136: f64, t243: f64, t10815: f64, t1561: f64, t10845: f64, t4430: f64, t1558: f64, t853: f64, t4353: f64, t9794: f64, t10760: f64, t10890: f64, t1549: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14685 = t243 * t136;
    let t14712 = t10815 * t1561;
    let t14716 = t10845 * t4430;
    let t14718 = t853 * t1558;
    let t14760 = t9794 * t4353;
    let t14761 = t10760 * t14760;
    let t14765 = t10890 * t1549;
    (t14685, t14712, t14716, t14718, t14760, t14761, t14765)
}
