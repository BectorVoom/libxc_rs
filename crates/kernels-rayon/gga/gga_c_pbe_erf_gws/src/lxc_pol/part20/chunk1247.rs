//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1247/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1247(t27047: f64, t3067: f64, t4164: f64, t814: f64, t9296: f64, t938: f64, t1112: f64, t361: f64, t51020: f64, t3209: f64, t51682: f64, t3958: f64, t6148: f64) -> (f64, f64, f64, f64, f64) {
    let t53790 = t27047 * t3067 * t4164 * t814;
    let t53795 = t27047 * t9296 * t4164 * t938;
    let t53799 = t361 * t51020 * t1112;
    let t53806 = t51682 * t3209;
    let t53807 = 7.0_f64 / 24.0_f64 * t53806;
    let t53840 = t3958 * t6148;
    (t53790, t53795, t53799, t53807, t53840)
}
