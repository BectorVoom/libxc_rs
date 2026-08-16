//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1253/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1253(t13972: f64, t14721: f64, t13808: f64, t14776: f64, t2306: f64, t3037: f64, t3074: f64, t331: f64, t833: f64, t14469: f64, t50884: f64, t13798: f64, t3972: f64, t50956: f64, t8827: f64) -> (f64, f64, f64, f64, f64) {
    let t53727 = t13972 * t14721;
    let t53729 = t13808 * t14776;
    let t53734 = t3074 * t2306 * t3037 * t331 * t833;
    let t53736 = t50884 * t14469;
    let t53742 = t3972 * t50956 * t8827 * t13798;
    (t53727, t53729, t53734, t53736, t53742)
}
