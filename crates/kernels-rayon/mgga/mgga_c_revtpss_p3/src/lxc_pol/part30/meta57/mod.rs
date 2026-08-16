//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta57 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk369;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk370;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk371;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk372;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk373;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk374;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk375;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta57(t1043: f64, t1089: f64, t378: f64, t1071: f64, t380: f64, t1024: f64, t1083: f64, t1087: f64, t342: f64, t381: f64, t989: f64, t1079: f64, t1000: f64, t1073: f64, t1076: f64, t386: f64, t995: f64, t389: f64, t30: f64, t265: f64, t393: f64, t198: f64, t336: f64, t895: f64, t912: f64, t938: f64, t978: f64, t980: f64, t985: f64, t395: f64, t45: f64, t605: f64, t606: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t268: f64, t404: f64, t900: f64, t159: f64, t482: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1090, t1093, t1096) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk369(t1043, t1089, t378, t1071, t380, t1024, t1083, t1087, t342, t381, t989);
        let t1097 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk370(t1079, t1096);
        let (t1100, t1102) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk371(t1000, t1073, t1076, t1097, t342, t386, t989, t995, t389);
        let (t1106, t1111) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk372(t30, t265, t393, t1100, t1102, t198, t336, t895, t912, t938, t978, t980, t985, t395, t45, t605, t606, dens_threshold, rho0, zeta_threshold);
        let t1113 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk373(t605);
        let t1118 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk374(t268, t404, t900);
        let (t1119, t1120) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk375(t1118, t159, t482);
    (t1090, t1093, t1096, t1097, t1100, t1102, t1106, t1111, t1113, t1118, t1119, t1120)
}
