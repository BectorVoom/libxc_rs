//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta174 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk888;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk889;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk890;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk891;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk892;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk893;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk894;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta174<F: Float>(t3601: F, t487: F, t3303: F, t3603: F, t1248: F, t1269: F, t1287: F, t3588: F, t1243: F, t3140: F, t460: F, t471: F, t3727: F, t489: F, t1204: F, t1234: F, t1281: F, t1285: F, t1288: F, t1291: F, t3552: F, t3666: F, t3670: F, t3746: F, t3751: F, t3755: F, t3756: F, t3760: F, t3763: F, t3767: F, t490: F, t1277: F, t1210: F, t1215: F, t1271: F, t1274: F, t1295: F, t3556: F, t3561: F, t3567: F, t3569: F, t3572: F, t3576: F, t3585: F, t3729: F, t3732: F, t3739: F, t495: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3768, t3769) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk888::<F>(t3601, t487, t3303, t3603);
        let (t3770, t3774, t3778, t3781) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk889::<F>(t3768, t3769, t1248, t1269, t1287, t3588, t487, t1243, t3140);
        let t3782 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk890::<F>(t3781, t460);
        let t3783 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk891::<F>(t3303, t471);
        let (t3784, t3787, t3790) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk892::<F>(t3768, t3783, t3727, t489, t1204, t1234, t1281, t1285, t1288, t1291, t3552, t3666, t3670, t3746, t3751, t3755, t3756, t3760, t3763, t3767, t3770, t3774, t3778, t3782, t460, t490);
        let t3791 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk893::<F>(t1277, t3790);
        let t3794 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk894::<F>(t1204, t1210, t1215, t1271, t1274, t1295, t3552, t3556, t3561, t3567, t3569, t3572, t3576, t3585, t3729, t3732, t3739, t3791, t460, t495);
    (t3769, t3770, t3774, t3778, t3781, t3782, t3783, t3784, t3787, t3790, t3791, t3794)
}
