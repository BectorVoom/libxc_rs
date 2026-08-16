//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta237 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1006;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1007;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1008;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1009;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1010;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1011;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta237(t3362: f64, t5819: f64, t3360: f64, t128: f64, t3367: f64, t1120: f64, t1121: f64, t5825: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t6421 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1006(t3362, t5819);
        let (t6422, t6423) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1007(t3360, t6421, t128);
        let t6425 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1008(t3367, t5819);
        let (t6426, t6427) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1009(t1120, t6425, t128);
        let t6429 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1010(t1121, t5825);
        let (t6430, t6431) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1011(t1120, t6429, t128);
    (t6421, t6422, t6423, t6425, t6426, t6427, t6429, t6430, t6431)
}
