//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta451 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1718;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1719;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1720;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta451<F: Float>(t1285: F, t12865: F, t372: F, t5302: F, t4181: F, t5405: F, t13396: F, t1042: F, t3588: F, t3603: F, t5332: F, t3720: F, t15904: F, t3623: F, t13148: F, t11249: F, t1794: F, t13045: F, t3601: F, t1261: F, t12784: F, t17669: F, t17674: F, t17679: F, t17684: F, t17690: F, t3625: F, t3708: F, t5287: F, t5331: F, t5340: F, t5407: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t17693, t17695, t17696, t17700, t17705) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1718::<F>(t1285, t12865, t372, t5302, t4181, t5405, t13396, t1042, t3588, t3603, t5332, t3720);
        let (t17708, t17709, t17710) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1719::<F>(t15904, t3623, t13148, t11249, t1794);
        let (t17713, t17718) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1720::<F>(t13045, t3601, t17710, t3720, t1261, t12784, t17669, t17674, t17679, t17684, t17690, t17693, t17696, t17700, t17705, t17709, t3625, t3708, t5287, t5331, t5340, t5407);
    (t17693, t17695, t17696, t17700, t17705, t17708, t17710, t17713, t17718)
}
