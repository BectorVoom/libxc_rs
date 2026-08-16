//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta65 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk419;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk420;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk421;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk422;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk423;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta65(t1214: f64, t482: f64, t372: f64, t371: f64, t1032: f64, t460: f64, t472: f64, t474: f64, t1038: f64, t479: f64, t1128: f64, t1153: f64, t1193: f64, t1195: f64, t1200: f64, t471: f64, t73: f64, t1042: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1236, t1238, t1241, t1242, t1243, t1244, t1246) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk419(t1214, t482, t372, t371, t1032, t460, t472, t474, t1038, t479);
        let t1247 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk420(t1241, t1246);
        let t1248 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk421(t1128, t1153, t1193, t1195, t1200);
        let t1250 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk422(t471, t73);
        let (t1251, t1252) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk423(t1248, t1250, t482, t1042);
    (t1236, t1238, t1241, t1242, t1243, t1244, t1246, t1247, t1248, t1250, t1251, t1252)
}
