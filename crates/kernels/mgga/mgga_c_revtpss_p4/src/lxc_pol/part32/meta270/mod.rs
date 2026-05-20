//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta270 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1139;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1140;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1141;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1142;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1143;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1144;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta270<F: Float>(t7076: F, t8011: F, t233: F, t7997: F, t1957: F, t1580: F, t1956: F, t2067: F, t213: F, t257: F, t7070: F, t7387: F, t7390: F, t7403: F, t7409: F, t7411: F, t7766: F, t7998: F, t8007: F, t892: F, t30: F, t265: F, t393: F, t1544: F, t2071: F, t207: F, t1583: F, t1940: F, t198: F, t2403: F, t7432: F, t1468: F, t1469: F, t2078: F, t45: F, t7787: F, t7991: F, dens_threshold: F, rho0: F, zeta_threshold: F, t33: F, t502: F, t7862: F, t1711: F, t2085: F, t57: F, t7869: F, rho1: F, t1312: F, t1518: F, t2055: F, t4248: F, t7359: F, t7889: F, t7969: F, t7983: F, t7488: F, t7900: F, t7499: F, t7501: F, t7502: F, t7504: F, t7904: F, t7906: F, t7908: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t8012, t8015, t8016, t8019) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1139::<F>(t7076, t8011, t233, t7997, t1957, t1580, t1956, t2067, t213, t257, t7070, t7387, t7390, t7403, t7409, t7411, t7766, t7998, t8007);
        let t8020 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1140::<F>(t8019, t892);
        let (t8034, t8039, t8040, t8045) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1141::<F>(t30, t265, t393, t1544, t2071, t207, t8019, t1583, t1940, t198, t2403, t7432, t892, t1468, t1469, t2078, t45, t7787, t7991, t8020, dens_threshold, rho0, zeta_threshold);
        let (t8059, t8064) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1142::<F>(t33, t265, t502, t2071, t7862, t8039, t1469, t1711, t1940, t2085, t2403, t57, t7432, t7869, t8020, dens_threshold, rho1, zeta_threshold);
        let t8065 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1143::<F>(t8045, t8064);
        let (t8075, t8079, t8085) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1144::<F>(t1312, t1518, t2055, t4248, t7359, t7889, t7969, t7983, t7488, t7900, t7499, t7501, t7502, t7504, t7904, t7906, t7908);
    (t8012, t8015, t8016, t8019, t8020, t8034, t8040, t8059, t8065, t8075, t8079, t8085)
}
