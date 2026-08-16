//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1233/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1233(t6683: f64, t904: f64, t1123: f64, t51989: f64, t833: f64, t850: f64, t13972: f64, t14721: f64, t13808: f64, t14776: f64, t51651: f64, t14135: f64, t3039: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53710 = t904 * t6683;
    let t53725 = t850 * t1123 * t51989 * t833;
    let t53727 = t13972 * t14721;
    let t53729 = t13808 * t14776;
    let t53750 = 35.0_f64 / 108.0_f64 * t51651;
    let t53774 = t3039 * t14135;
    (t53710, t53725, t53727, t53729, t53750, t53774)
}
