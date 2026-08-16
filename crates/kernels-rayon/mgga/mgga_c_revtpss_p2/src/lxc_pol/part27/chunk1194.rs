//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1194/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1194(t25373: f64, t25392: f64, t25386: f64, t268: f64, t41040: f64, t837: f64, t25372: f64, t25287: f64, t786: f64, t789: f64, t2829: f64, t689: f64, t7014: f64) -> (f64, f64, f64, f64) {
    let t92837 = t25373 * t25392;
    let t92838 = t25386 * t92837;
    let t92840 = t268 * t41040 * t837;
    let t92841 = t92838 * t92840;
    let t92843 = t25372 * t92837;
    let t92844 = t92843 * t92840;
    let t92847 = t786 * t25287 * t789;
    let t92856 = t689 * t7014 * t2829;
    (t92841, t92844, t92847, t92856)
}
