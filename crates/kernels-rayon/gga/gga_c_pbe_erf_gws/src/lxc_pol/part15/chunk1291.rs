//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1291/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1291(t3959: f64, t8797: f64, t14121: f64, t8624: f64, t14001: f64, t14463: f64, t2409: f64, t2417: f64, t3066: f64, t4182: f64, t53950: f64, t53953: f64, t53959: f64, t53963: f64, t53966: f64, t53968: f64, t53971: f64, t53973: f64, t53976: f64, t53977: f64, t53980: f64, t9296: f64) -> f64 {
    let t53981 = t3959 * t8797;
    let t53983 = t14121 * t8624;
    let t53985 = t14001 * t14463;
    let t53986 = 7.0_f64 / 72.0_f64 * t53985;
    let t53987 = t53950 / 24.0_f64 + t53953 - t3066 * t2409 * t9296 * t4182 * t2417 / 16.0_f64 + 35.0_f64 / 216.0_f64 * t53959 + 5.0_f64 / 384.0_f64 * t53963 - t53966 / 48.0_f64 + t53968 / 24.0_f64 - t53971 + t53973 / 16.0_f64 + t53976 - 35.0_f64 / 432.0_f64 * t53977 + t53980 + t53981 / 24.0_f64 + t53983 / 8.0_f64 + t53986;
    t53987
}
