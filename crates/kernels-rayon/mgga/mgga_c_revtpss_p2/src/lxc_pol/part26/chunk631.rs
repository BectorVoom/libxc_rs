//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 631/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk631(t1370: f64, t1388: f64, t3926: f64, t3931: f64, t3934: f64, t3940: f64, t3944: f64, t3946: f64, t3950: f64, t3953: f64, t3956: f64, t3958: f64, t3961: f64, t3967: f64, t4065: f64) -> f64 {
    let t4066 = -0.21437009059034868486e-3_f64 * t1388 * t3926 + 0.20007875121765877254e-2_f64 * t3931 + 0.17149607247227894789e-2_f64 * t3934 * t3940 + t3944 * t3946 / 16.0_f64 + t3950 + 0.57165357490759649296e-4_f64 * t3953 + t3956 + 7.0_f64 / 72.0_f64 * t3958 - t1370 * t3961 / 48.0_f64 + t3967 + t4065;
    t4066
}
