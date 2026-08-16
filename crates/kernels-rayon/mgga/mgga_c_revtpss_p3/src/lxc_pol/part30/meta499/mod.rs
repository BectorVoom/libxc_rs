//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta499 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1856;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1857;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1858;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1859;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1860;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1861;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1862;
use chunk7::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1863;
use chunk8::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1864;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta499(t26865: f64, t3089: f64, t1285: f64, t3717: f64, t3707: f64, t7617: f64, t2134: f64, t3682: f64, t1234: f64, t7623: f64, t1252: f64, t1266: f64, t26849: f64, t26852: f64, t26855: f64, t26863: f64, t3591: f64, t3613: f64, t3620: f64, t3631: f64, t3640: f64, t3644: f64, t3714: f64, t3723: f64, t7618: f64, t7624: f64, t26847: f64, t225: f64, t494: f64, t1210: f64, t8945: f64, t1248: f64, t1287: f64, t7638: f64, t487: f64, t7642: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t26866 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1856(t26865, t3089);
        let t26867 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1857(t1285, t26866);
        let t26870 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1858(t26866, t3717);
        let t26873 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1859(t3707, t7617);
        let (t26877, t26880) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1860(t2134, t3682, t1234, t7623);
        let t26883 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1861(t1252, t1266, t26849, t26852, t26855, t26863, t26867, t26870, t26873, t26877, t26880, t3591, t3613, t3620, t3631, t3640, t3644, t3714, t3723, t7618, t7624);
        let t26884 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1862(t26847, t26883);
        let (t26886, t26889) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1863(t225, t26884, t494, t1210, t8945);
        let (t26891, t26894, t26895) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1864(t1248, t1287, t7638, t487, t7642, t8945);
    (t26866, t26867, t26870, t26873, t26877, t26880, t26884, t26886, t26889, t26891, t26894, t26895)
}
