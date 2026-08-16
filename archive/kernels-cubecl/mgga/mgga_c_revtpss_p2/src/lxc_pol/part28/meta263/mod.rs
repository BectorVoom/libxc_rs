//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta263 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1176;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1177;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1178;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1179;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1180;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1181;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1182;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta263<F: Float>(t351: F, t7125: F, t1058: F, t1973: F, t1061: F, t1971: F, t1017: F, t1028: F, t1047: F, t1068: F, t348: F, t375: F, t7106: F, t7110: F, t7111: F, t7114: F, t7117: F, t7122: F, t225: F, t385: F, t1976: F, t342: F, t1032: F, t378: F, t994: F, t1078: F, t359: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t7126, t7130, t7131) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1176::<F>(t351, t7125, t1058, t1973, t1061, t1971);
        let t7132 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1177::<F>(t351, t7131);
        let t7135 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1178::<F>(t1017, t1028, t1047, t1068, t348, t375, t7106, t7110, t7111, t7114, t7117, t7122, t7126, t7130, t7132);
        let (t7137, t7140) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1179::<F>(t225, t385, t7135, t1976, t342);
        let t7143 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1180::<F>(t1032, t378);
        let t7144 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1181::<F>(t7143, t994);
        let t7145 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1182::<F>(t1078, t359);
    (t7126, t7130, t7131, t7132, t7135, t7137, t7140, t7143, t7144, t7145)
}
