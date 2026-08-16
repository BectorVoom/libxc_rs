//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta62 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk400;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk401;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk402;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk403;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk404;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta62<F: Float>(t1128: F, t1153: F, t1156: F, t1161: F, t1170: F, t1176: F, t1180: F, t1189: F, t300: F, t435: F, t439: F, t1179: F, t1187: F, t1188: F, t1118: F, t1124: F, t459: F, t458: F, t456: F, t487: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1193, t1195, t1196) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk400::<F>(t1128, t1153, t1156, t1161, t1170, t1176, t1180, t1189, t300, t435, t439);
        let (t1198, t1200, t1201, t1203, t1204) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk401::<F>(t1179, t1187, t1188, t1196, t1118, t1124, t459);
        let (t1207, t1208) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk402::<F>(t458);
        let t1209 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk403::<F>(t1208, t456);
        let t1210 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk404::<F>(t1209, t487);
    (t1193, t1195, t1196, t1198, t1200, t1201, t1203, t1204, t1207, t1208, t1209, t1210)
}
