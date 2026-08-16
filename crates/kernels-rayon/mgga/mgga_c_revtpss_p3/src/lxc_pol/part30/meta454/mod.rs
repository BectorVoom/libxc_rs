//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta454 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1728;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1729;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1730;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1731;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta454(t17807: f64, t489: f64, t3759: f64, t5230: f64, t1811: f64, t3601: f64, t3769: f64, t16695: f64, t17454: f64, t473: f64, t5412: f64, t1214: f64, t1269: f64, t1287: f64, t5284: f64, t17633: f64, t5458: f64, t17482: f64, t3783: f64, t12713: f64, t5332: f64, t13147: f64, t487: f64, t460: f64, t12050: f64, t13045: f64, t17710: f64, t13141: f64, t3603: f64, t1234: f64, t12717: f64, t12751: f64, t12756: f64, t1285: f64, t12966: f64, t12975: f64, t17188: f64, t17192: f64, t1818: f64, t3666: f64, t3670: f64, t3755: f64, t3756: f64, t3767: f64, t5443: f64, t5452: f64, t5463: f64, t1284: f64, t5216: f64, t1204: f64, t5477: f64, t3302: f64, t3588: f64, t471: f64, t3781: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17808, t17811, t17814, t17815, t17818, t17822) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1728(t17807, t489, t3759, t5230, t1811, t3601, t3769, t16695, t17454, t473, t5412, t1214);
        let (t17826, t17829, t17834, t17837, t17840, t17845) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1729(t1269, t1287, t5284, t17633, t5458, t17482, t3769, t3783, t12713, t5332, t13147, t487);
        let (t17848, t17855, t17859) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1730(t17845, t460, t12050, t13045, t3601, t17710, t13141, t487, t3603, t1234, t12717, t12751, t12756, t1285, t12966, t12975, t17188, t17192, t17808, t17811, t17815, t17818, t17822, t17826, t17829, t17834, t17837, t17840, t1818, t3666, t3670, t3755, t3756, t3767, t5443, t5452, t5463);
        let (t17861, t17864, t17869, t17875, t17876, t17879) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1731(t1284, t5216, t1204, t5477, t17814, t3783, t3302, t3588, t471, t5332, t1269, t3781);
    (t17848, t17855, t17859, t17861, t17864, t17869, t17875, t17876, t17879)
}
