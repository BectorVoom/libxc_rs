//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta454 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1728;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1729;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1730;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1731;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta454<F: Float>(t17807: F, t489: F, t3759: F, t5230: F, t1811: F, t3601: F, t3769: F, t16695: F, t17454: F, t473: F, t5412: F, t1214: F, t1269: F, t1287: F, t5284: F, t17633: F, t5458: F, t17482: F, t3783: F, t12713: F, t5332: F, t13147: F, t487: F, t460: F, t12050: F, t13045: F, t17710: F, t13141: F, t3603: F, t1234: F, t12717: F, t12751: F, t12756: F, t1285: F, t12966: F, t12975: F, t17188: F, t17192: F, t1818: F, t3666: F, t3670: F, t3755: F, t3756: F, t3767: F, t5443: F, t5452: F, t5463: F, t1284: F, t5216: F, t1204: F, t5477: F, t3302: F, t3588: F, t471: F, t3781: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t17808, t17811, t17814, t17815, t17818, t17822) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1728::<F>(t17807, t489, t3759, t5230, t1811, t3601, t3769, t16695, t17454, t473, t5412, t1214);
        let (t17826, t17829, t17834, t17837, t17840, t17845) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1729::<F>(t1269, t1287, t5284, t17633, t5458, t17482, t3769, t3783, t12713, t5332, t13147, t487);
        let (t17848, t17855, t17859) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1730::<F>(t17845, t460, t12050, t13045, t3601, t17710, t13141, t487, t3603, t1234, t12717, t12751, t12756, t1285, t12966, t12975, t17188, t17192, t17808, t17811, t17815, t17818, t17822, t17826, t17829, t17834, t17837, t17840, t1818, t3666, t3670, t3755, t3756, t3767, t5443, t5452, t5463);
        let (t17861, t17864, t17869, t17875, t17876, t17879) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1731::<F>(t1284, t5216, t1204, t5477, t17814, t3783, t3302, t3588, t471, t5332, t1269, t3781);
    (t17848, t17855, t17859, t17861, t17864, t17869, t17875, t17876, t17879)
}
