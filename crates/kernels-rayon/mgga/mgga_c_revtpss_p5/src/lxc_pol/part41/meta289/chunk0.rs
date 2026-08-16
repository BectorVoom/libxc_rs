//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1048/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1048(t10981: f64, t10982: f64, t2455: f64, t9285: f64, t2454: f64, t252: f64, t2769: f64, t786: f64, t2435: f64, t2448: f64, t2440: f64, t887: f64) -> (f64, f64, f64, f64, f64) {
    let t10984 = 0.19637199382202157274e-3_f64 * t10981 * t10982;
    let t10985 = t2455 * t9285;
    let t10987 = 0.46263278077393568556e-2_f64 * t2454 * t10985;
    let t10994 = t252 * t2769;
    let t10995 = t786 * t10994;
    let t11000 = t2435 * t2448;
    let t11003 = t2440 * t887;
    (t10984, t10987, t10995, t11000, t11003)
}
