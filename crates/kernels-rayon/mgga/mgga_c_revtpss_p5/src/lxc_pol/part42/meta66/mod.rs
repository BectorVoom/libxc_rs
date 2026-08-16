//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta66 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk399;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk400;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk401;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk402;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk403;
use chunk5::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk404;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta66(t1248: f64, t1287: f64, t487: f64, t1269: f64, t489: f64, t1204: f64, t1234: f64, t1281: f64, t1285: f64, t460: f64, t490: f64, t1277: f64, t1210: f64, t1215: f64, t1271: f64, t1274: f64, t495: f64, t498: f64, t33: f64, t265: f64, t502: f64, t1128: f64, t1153: f64, t1193: f64, t1195: f64, t1200: f64, t198: f64, t336: f64, t895: f64, t1113: f64, t504: f64, t57: f64, t606: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t1111: f64, t116: f64, t93: f64, t649: f64, t670: f64, t22: f64, t583: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1288, t1291, t1294) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk399(t1248, t1287, t487, t1269, t489, t1204, t1234, t1281, t1285, t460, t490);
        let (t1295, t1298, t1300) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk400(t1277, t1294, t1204, t1210, t1215, t1271, t1274, t460, t495, t498);
        let (t1304, t1309) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk401(t33, t265, t502, t1128, t1153, t1193, t1195, t1200, t1298, t1300, t198, t336, t895, t1113, t504, t57, t606, dens_threshold, rho1, zeta_threshold);
        let t1310 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk402(t1111, t1309);
        let t1312 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk403(t116, t93);
        let (t1315, t1317) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk404(t1312, t649, t670, t22, t583);
    (t1288, t1291, t1294, t1295, t1298, t1300, t1304, t1310, t1312, t1315, t1317)
}
