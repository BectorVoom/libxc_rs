//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta68 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk425;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk426;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk427;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk428;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk429;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk430;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk431;
use chunk7::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk432;
use chunk8::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk433;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta68(t1204: f64, t1210: f64, t1215: f64, t1271: f64, t1274: f64, t1295: f64, t460: f64, t495: f64, t498: f64, t33: f64, t265: f64, t502: f64, t1128: f64, t1153: f64, t1193: f64, t1195: f64, t1200: f64, t198: f64, t336: f64, t895: f64, t1113: f64, t504: f64, t57: f64, t606: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t1111: f64, t116: f64, t93: f64, t649: f64, t670: f64, t22: f64, t583: f64, t521: f64, t19: f64, t588: f64, t30: f64, t513: f64, t605: f64, t516: f64, t162: f64, t189: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1298, t1300) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk425(t1204, t1210, t1215, t1271, t1274, t1295, t460, t495, t498);
        let (t1304, t1309) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk426(t33, t265, t502, t1128, t1153, t1193, t1195, t1200, t1298, t1300, t198, t336, t895, t1113, t504, t57, t606, dens_threshold, rho1, zeta_threshold);
        let t1310 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk427(t1111, t1309);
        let t1312 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk428(t116, t93);
        let t1315 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk429(t1312, t649, t670);
        let t1317 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk430(t22, t583);
        let (t1319, t1320) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk431(t1317, t521, t19, t588);
        let (t1322, t1330) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk432(t30, t33, t1320, t521, t513, t605, t1113, t516, t162, zeta_threshold);
        let t1331 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk433(t1330, t189);
    (t1298, t1300, t1304, t1310, t1312, t1315, t1317, t1319, t1320, t1322, t1330, t1331)
}
