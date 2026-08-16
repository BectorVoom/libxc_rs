//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1245/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1245(t14420: f64, t19906: f64, t6683: f64, t904: f64, t1123: f64, t51989: f64, t833: f64, t850: f64, t13972: f64, t14721: f64, t13808: f64, t14776: f64) -> (f64, f64, f64, f64, f64) {
    let t53704 = 7.0_f64 / 72.0_f64 * t19906 * t14420;
    let t53710 = t904 * t6683;
    let t53725 = t850 * t1123 * t51989 * t833;
    let t53726 = 7.0_f64 / 144.0_f64 * t53725;
    let t53727 = t13972 * t14721;
    let t53728 = 7.0_f64 / 2304.0_f64 * t53727;
    let t53729 = t13808 * t14776;
    (t53704, t53710, t53726, t53728, t53729)
}
