//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta174 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk888;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk889;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk890;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk891;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk892;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk893;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk894;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta174(t3601: f64, t487: f64, t3303: f64, t3603: f64, t1248: f64, t1269: f64, t1287: f64, t3588: f64, t1243: f64, t3140: f64, t460: f64, t471: f64, t3727: f64, t489: f64, t1204: f64, t1234: f64, t1281: f64, t1285: f64, t1288: f64, t1291: f64, t3552: f64, t3666: f64, t3670: f64, t3746: f64, t3751: f64, t3755: f64, t3756: f64, t3760: f64, t3763: f64, t3767: f64, t490: f64, t1277: f64, t1210: f64, t1215: f64, t1271: f64, t1274: f64, t1295: f64, t3556: f64, t3561: f64, t3567: f64, t3569: f64, t3572: f64, t3576: f64, t3585: f64, t3729: f64, t3732: f64, t3739: f64, t495: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3768, t3769) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk888(t3601, t487, t3303, t3603);
        let (t3770, t3774, t3778, t3781) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk889(t3768, t3769, t1248, t1269, t1287, t3588, t487, t1243, t3140);
        let t3782 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk890(t3781, t460);
        let t3783 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk891(t3303, t471);
        let (t3784, t3787, t3790) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk892(t3768, t3783, t3727, t489, t1204, t1234, t1281, t1285, t1288, t1291, t3552, t3666, t3670, t3746, t3751, t3755, t3756, t3760, t3763, t3767, t3770, t3774, t3778, t3782, t460, t490);
        let t3791 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk893(t1277, t3790);
        let t3794 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk894(t1204, t1210, t1215, t1271, t1274, t1295, t3552, t3556, t3561, t3567, t3569, t3572, t3576, t3585, t3729, t3732, t3739, t3791, t460, t495);
    (t3769, t3770, t3774, t3778, t3781, t3782, t3783, t3784, t3787, t3790, t3791, t3794)
}
