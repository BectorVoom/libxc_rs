//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta266 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1187;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1188;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1189;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1190;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1191;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1192;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta266(t1372: f64, t7252: f64, t546: f64, t550: f64, t7028: f64, t807: f64, t2018: f64, t786: f64, t1381: f64, t1385: f64, t64: f64, t239: f64, t820: f64, t1401: f64, t1405: f64, t2019: f64, t545: f64, t1416: f64, t7251: f64, t225: f64, t2022: f64, t213: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7253, t7256, t7258, t7259, t7261, t7262) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1187(t1372, t7252, t546, t550, t7028, t807, t2018, t786, t1381, t1385, t64);
        let t7264 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1188(t239, t7262, t820);
        let (t7265, t7268, t7269) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1189(t1401, t7264, t1405, t2019, t545, t64);
        let t7271 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1190(t239, t7269, t820);
        let t7274 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1191(t1416, t7271, t7251, t7253, t7258, t7261, t7265, t7268);
        let (t7275, t7279) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1192(t225, t7274, t2022, t213);
    (t7256, t7258, t7259, t7261, t7262, t7264, t7268, t7269, t7271, t7274, t7275, t7279)
}
