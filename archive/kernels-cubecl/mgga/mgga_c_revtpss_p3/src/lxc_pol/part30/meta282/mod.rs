//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta282 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1236;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1237;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1238;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1239;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1240;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta282<F: Float>(t30: F, t1469: F, t2129: F, t45: F, t7794: F, t8161: F, t1479: F, t343: F, t136: F, t1785: F, t2138: F, t1802: F, t2137: F, dens_threshold: F, rho0: F, zeta_threshold: F, t467: F, t1782: F, t1791: F, t1797: F, t1808: F, t464: F, t484: F, t7606: F, t7607: F, t7613: F, t7618: F, t7622: F, t7624: F, t225: F, t494: F, t1769: F, t2142: F, t7637: F, t1774: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t8166, t8171, t8172, t8177, t8184) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1236::<F>(t30, t1469, t2129, t45, t7794, t8161, t1479, t343, t136, t1785, t2138, t1802, t2137, dens_threshold, rho0, zeta_threshold);
        let (t8185, t8190) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1237::<F>(t467, t8184, t1782, t1791, t1797, t1808, t464, t484, t7606, t7607, t7613, t7618, t7622, t7624, t8172, t8177);
        let (t8192, t8197) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1238::<F>(t225, t494, t8190, t1769, t2142);
        let t8198 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1239::<F>(t7637, t8197);
        let t8201 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1240::<F>(t1774, t2142);
    (t8166, t8171, t8172, t8177, t8184, t8185, t8190, t8192, t8197, t8198, t8201)
}
