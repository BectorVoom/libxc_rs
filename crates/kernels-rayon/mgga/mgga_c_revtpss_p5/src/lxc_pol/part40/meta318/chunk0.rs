//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1093/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1093(t3241: f64, t3244: f64, t1058: f64, t3197: f64, t11132: f64, t3163: f64, t3172: f64, t3161: f64, t126: f64, t373: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11886 = t3241 * t3244;
    let t11888 = t3197 * t1058;
    let t11890 = 0.25925925925925925926e-1_f64 * t11132;
    let t11916 = t3172 * t3163;
    let t11917 = t3161 * t11916;
    let t11921 = t126 * t373;
    let t11922 = t828 * t11921;
    (t11886, t11888, t11890, t11917, t11921, t11922)
}
