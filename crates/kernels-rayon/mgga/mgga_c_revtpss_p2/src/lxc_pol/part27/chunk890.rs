//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 890/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk890(t10741: f64, t2674: f64, t2735: f64, t2783: f64, t2664: f64, t808: f64, t2693: f64, t2710: f64, t2713: f64, t2706: f64, t775: f64, t800: f64) -> (f64, f64, f64, f64, f64) {
    let t10742 = t2674 * t10741;
    let t10744 = t2735 * t2783;
    let t10745 = t808 * t2664;
    let t10746 = t10744 * t10745;
    let t10749 = t2710 * t2713 * t2693;
    let t10752 = t800 * t2706 * t775;
    (t10742, t10744, t10746, t10749, t10752)
}
