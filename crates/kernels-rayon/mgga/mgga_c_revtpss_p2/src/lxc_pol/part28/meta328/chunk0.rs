//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1342/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1342(t10981: f64, t10982: f64, t2455: f64, t9285: f64, t2454: f64, t2829: f64, t779: f64, t689: f64, t2444: f64, t887: f64, t252: f64, t2769: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10984 = 0.19637199382202157274e-3_f64 * t10981 * t10982;
    let t10985 = t2455 * t9285;
    let t10987 = 0.46263278077393568556e-2_f64 * t2454 * t10985;
    let t10988 = t779 * t2829;
    let t10989 = t689 * t10988;
    let t10991 = t2444 * t887;
    let t10992 = t689 * t10991;
    let t10994 = t252 * t2769;
    (t10984, t10985, t10987, t10989, t10992, t10994)
}
