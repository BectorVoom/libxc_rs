//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta451 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1718;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1719;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1720;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta451(t1285: f64, t12865: f64, t372: f64, t5302: f64, t4181: f64, t5405: f64, t13396: f64, t1042: f64, t3588: f64, t3603: f64, t5332: f64, t3720: f64, t15904: f64, t3623: f64, t13148: f64, t11249: f64, t1794: f64, t13045: f64, t3601: f64, t1261: f64, t12784: f64, t17669: f64, t17674: f64, t17679: f64, t17684: f64, t17690: f64, t3625: f64, t3708: f64, t5287: f64, t5331: f64, t5340: f64, t5407: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17693, t17695, t17696, t17700, t17705) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1718(t1285, t12865, t372, t5302, t4181, t5405, t13396, t1042, t3588, t3603, t5332, t3720);
        let (t17708, t17709, t17710) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1719(t15904, t3623, t13148, t11249, t1794);
        let (t17713, t17718) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1720(t13045, t3601, t17710, t3720, t1261, t12784, t17669, t17674, t17679, t17684, t17690, t17693, t17696, t17700, t17705, t17709, t3625, t3708, t5287, t5331, t5340, t5407);
    (t17693, t17695, t17696, t17700, t17705, t17708, t17710, t17713, t17718)
}
