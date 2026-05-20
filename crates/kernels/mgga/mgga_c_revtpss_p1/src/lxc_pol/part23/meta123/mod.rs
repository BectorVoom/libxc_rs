//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta123 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk802;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk803;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk804;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk805;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk806;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk807;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk808;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk809;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta123<F: Float>(t3356: F, t1123: F, t689: F, t1263: F, t159: F, t635: F, t2304: F, t1126: F, t1130: F, t1129: F, t418: F, t408: F, t406: F, t409: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3357, t3358) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk802::<F>(t3356, t1123, t689);
        let t3360 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk803::<F>(t1263, t159);
        let t3361 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk804::<F>(t635);
        let t3362 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk805::<F>(t3361);
        let t3367 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk806::<F>(t2304);
        let (t3379, t3382, t3383) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk807::<F>(t1126, t1130, t1129, t418);
        let t3384 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk808::<F>(t3383, t408);
        let t3390 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk809::<F>(t406, t409);
    (t3357, t3358, t3360, t3361, t3362, t3367, t3379, t3382, t3383, t3384, t3390)
}
