//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta272 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1201;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1202;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1203;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1204;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1205;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta272<F: Float>(t1203: F, t2142: F, t7637: F, t2147: F, t3565: F, t7635: F, t1214: F, t1269: F, t2148: F, t3736: F, t473: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t7638, t7639, t7642) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1201::<F>(t1203, t2142, t7637, t2147, t3565);
        let t7643 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1202::<F>(t7635, t7642);
        let (t7644, t7645, t7648) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1203::<F>(t1214, t2142, t7637, t1269, t2148);
        let t7651 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1204::<F>(t2148, t7635);
        let t7652 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1205::<F>(t3736, t473);
    (t7638, t7639, t7642, t7643, t7644, t7645, t7648, t7651, t7652)
}
