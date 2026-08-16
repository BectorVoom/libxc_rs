//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta174 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk885;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk886;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk887;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk888;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk889;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta174(t1277: f64, t3790: f64, t1204: f64, t1210: f64, t1215: f64, t1271: f64, t1274: f64, t1295: f64, t3552: f64, t3556: f64, t3561: f64, t3567: f64, t3569: f64, t3572: f64, t3576: f64, t3585: f64, t3729: f64, t3732: f64, t3739: f64, t460: f64, t495: f64, t1298: f64, t498: f64, t1300: f64, t198: f64, t336: f64, t3378: f64, t3381: f64, t3388: f64, t3430: f64, t3438: f64, t3528: f64, t3530: f64, t3533: f64, t3537: f64, t3541: f64, t3545: f64, t33: f64, t265: f64, t502: f64, t2838: f64, t1113: f64, t1304: f64, t2258: f64, t3351: f64, t504: f64, t57: f64, t606: f64, t895: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t3347: f64, t1312: f64, t2320: f64, t2322: f64, t2327: f64, t2371: f64, t670: f64, t93: f64, t1330: f64, t72: f64, t757: f64, t530: f64, t566: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3791, t3794) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk885(t1277, t3790, t1204, t1210, t1215, t1271, t1274, t1295, t3552, t3556, t3561, t3567, t3569, t3572, t3576, t3585, t3729, t3732, t3739, t460, t495);
        let (t3798, t3800, t3801, t3804) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk886(t1298, t498, t1300, t198, t336, t3378, t3381, t3388, t3430, t3438, t3528, t3530, t3533, t3537, t3541, t3545, t3794);
        let (t3805, t3812) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk887(t33, t265, t502, t2838, t3804, t1113, t1304, t2258, t3351, t504, t57, t606, t895, dens_threshold, rho1, zeta_threshold);
        let t3813 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk888(t3347, t3812);
        let (t3821, t3825, t3826, t3827, t3828) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk889(t1312, t2320, t2322, t2327, t2371, t670, t93, t1330, t72, t757, t530, t566);
    (t3791, t3794, t3798, t3800, t3801, t3805, t3813, t3821, t3825, t3826, t3827, t3828)
}
