//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta57 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk371;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk372;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk373;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk374;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk375;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk376;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk377;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk378;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta57<F: Float>(t1043: F, t1089: F, t378: F, t1071: F, t380: F, t1024: F, t1083: F, t1087: F, t342: F, t381: F, t989: F, t1079: F, t1000: F, t1073: F, t1076: F, t386: F, t995: F, t389: F, t30: F, t265: F, t393: F, t198: F, t336: F, t895: F, t912: F, t938: F, t978: F, t980: F, t985: F, t395: F, t45: F, t605: F, t606: F, dens_threshold: F, rho0: F, zeta_threshold: F, t268: F, t404: F, t900: F, t159: F, t482: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1090, t1093, t1096) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk371::<F>(t1043, t1089, t378, t1071, t380, t1024, t1083, t1087, t342, t381, t989);
        let t1097 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk372::<F>(t1079, t1096);
        let t1100 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk373::<F>(t1000, t1073, t1076, t1097, t342, t386, t989, t995);
        let t1102 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk374::<F>(t389);
        let (t1106, t1111) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk375::<F>(t30, t265, t393, t1100, t1102, t198, t336, t895, t912, t938, t978, t980, t985, t395, t45, t605, t606, dens_threshold, rho0, zeta_threshold);
        let t1113 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk376::<F>(t605);
        let t1118 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk377::<F>(t268, t404, t900);
        let (t1119, t1120) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk378::<F>(t1118, t159, t482);
    (t1090, t1093, t1096, t1097, t1100, t1102, t1106, t1111, t1113, t1118, t1119, t1120)
}
