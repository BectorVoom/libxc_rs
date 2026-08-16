//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta171 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk869;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk870;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk871;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk872;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk873;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta171<F: Float>(t1224: F, t3367: F, t2251: F, t1012: F, t1121: F, t404: F, t3362: F, t1251: F, t3172: F, t1247: F, t1032: F, t1204: F, t1246: F, t1234: F, t1260: F, t1214: F, t1263: F, t1122: F, t1042: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3693, t3694, t3698) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk869::<F>(t1224, t3367, t2251, t1012, t1121, t404);
        let (t3700, t3701, t3704) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk870::<F>(t3362, t3698, t2251, t1012, t1251, t3172);
        let (t3705, t3707, t3708) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk871::<F>(t1247, t3704, t1032, t1204, t1246);
        let t3711 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk872::<F>(t1234, t1260);
        let (t3713, t3714) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk873::<F>(t1214, t1263, t1122, t1042);
    (t3693, t3694, t3698, t3700, t3701, t3704, t3705, t3707, t3708, t3711, t3713, t3714)
}
