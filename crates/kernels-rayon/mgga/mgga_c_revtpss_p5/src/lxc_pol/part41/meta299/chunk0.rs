//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1064/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1064(t11865: f64, t3090: f64, t3316: f64, t994: f64, t4891: f64, t1016: f64, t697: f64, t1011: f64, t11132: f64, t126: f64, t373: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11866 = t11865 * t3090;
    let t11874 = t994 * t3316;
    let t11875 = t11874 * t4891;
    let t11880 = t697 * t1016;
    let t11881 = t1011 * t11880;
    let t11890 = 0.25925925925925925926e-1_f64 * t11132;
    let t11921 = t126 * t373;
    let t11922 = t828 * t11921;
    (t11866, t11875, t11881, t11890, t11921, t11922)
}
