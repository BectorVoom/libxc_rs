//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta381 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1429;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1430;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1431;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1432;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta381(t9597: f64, t123: f64, t1856: f64, t2630: f64, t1857: f64, t3860: f64, t3863: f64, t13581: f64, t189: f64, t512: f64, t1907: f64, t9593: f64, t30: f64, t5566: f64, t749: f64, t9856: f64, t1468: f64, t9605: f64, t2: f64, t3874: f64, t1344: f64, t13554: f64, t22: f64, t2257: f64, t3834: f64, t5574: f64, t5577: f64, t580: f64, zeta_threshold: f64, t33: f64, t1711: f64, t9617: f64, t3881: f64, t1348: f64, t13569: f64, t3351: f64, t3842: f64, t5582: f64, t5585: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13664, t13667, t13669, t13671, t13673, t13674) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1429(t9597, t123, t1856, t2630, t1857, t3860, t3863, t13581, t189, t512, t1907, t9593);
        let (t13682, t13683, t13700) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1430(t30, t5566, t749, t512, t9856, t1468, t9605, t2, t3874, t1344, t13554, t22, t2257, t3834, t5574, t5577, t580, zeta_threshold);
        let t13714 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1431(t33, t1711, t9617, t2, t3881, t1348, t13569, t22, t3351, t3842, t5582, t5585, t580, zeta_threshold);
        let t13716 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1432(t13700, t13714);
    (t13664, t13667, t13669, t13671, t13673, t13674, t13682, t13683, t13716)
}
