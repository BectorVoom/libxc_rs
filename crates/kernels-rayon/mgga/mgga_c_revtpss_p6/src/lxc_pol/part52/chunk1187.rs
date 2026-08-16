//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1187/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1187(t31837: f64, t33695: f64, t31841: f64, t31838: f64, t33715: f64, t845: f64, t126138: f64, t2747: f64, t31767: f64, t31772: f64, t2769: f64, t34074: f64) -> (f64, f64, f64, f64) {
    let t126213 = t33695 * t31837;
    let t126214 = t126213 * t31841;
    let t126226 = t31838 * t845 * t33715;
    let t126232 = t31767 * t2747 * t31772 * t126138;
    let t126250 = t34074 * t2769;
    (t126214, t126226, t126232, t126250)
}
