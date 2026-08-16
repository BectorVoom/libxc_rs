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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta499<F: Float>(t26865: F, t3089: F, t1285: F, t3717: F, t3707: F, t7617: F, t2134: F, t3682: F, t1234: F, t7623: F, t1252: F, t1266: F, t26849: F, t26852: F, t26855: F, t26863: F, t3591: F, t3613: F, t3620: F, t3631: F, t3640: F, t3644: F, t3714: F, t3723: F, t7618: F, t7624: F, t26847: F, t225: F, t494: F, t1210: F, t8945: F, t1248: F, t1287: F, t7638: F, t487: F, t7642: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t26866 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1856::<F>(t26865, t3089);
        let t26867 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1857::<F>(t1285, t26866);
        let t26870 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1858::<F>(t26866, t3717);
        let t26873 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1859::<F>(t3707, t7617);
        let (t26877, t26880) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1860::<F>(t2134, t3682, t1234, t7623);
        let t26883 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1861::<F>(t1252, t1266, t26849, t26852, t26855, t26863, t26867, t26870, t26873, t26877, t26880, t3591, t3613, t3620, t3631, t3640, t3644, t3714, t3723, t7618, t7624);
        let t26884 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1862::<F>(t26847, t26883);
        let (t26886, t26889) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1863::<F>(t225, t26884, t494, t1210, t8945);
        let (t26891, t26894, t26895) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1864::<F>(t1248, t1287, t7638, t487, t7642, t8945);
    (t26866, t26867, t26870, t26873, t26877, t26880, t26884, t26886, t26889, t26891, t26894, t26895)
}
