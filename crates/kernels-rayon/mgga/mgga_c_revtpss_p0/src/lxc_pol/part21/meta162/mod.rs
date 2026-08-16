//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta162 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1031;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1032;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1033;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1034;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1035;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1036;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1037;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta162(t3768: f64, t3783: f64, t3727: f64, t489: f64, t1204: f64, t1234: f64, t1281: f64, t1285: f64, t1288: f64, t1291: f64, t3552: f64, t3666: f64, t3670: f64, t3746: f64, t3751: f64, t3755: f64, t3756: f64, t3760: f64, t3763: f64, t3767: f64, t3770: f64, t3774: f64, t3778: f64, t3782: f64, t460: f64, t490: f64, t1277: f64, t1210: f64, t1215: f64, t1271: f64, t1274: f64, t1295: f64, t3556: f64, t3561: f64, t3567: f64, t3569: f64, t3572: f64, t3576: f64, t3585: f64, t3729: f64, t3732: f64, t3739: f64, t495: f64, t1298: f64, t498: f64, t1300: f64, t198: f64, t336: f64, t3378: f64, t3381: f64, t3388: f64, t3430: f64, t3438: f64, t3528: f64, t3530: f64, t3533: f64, t3537: f64, t3541: f64, t3545: f64, t33: f64, t265: f64, t502: f64, t2838: f64, t1113: f64, t1304: f64, t2258: f64, t3351: f64, t504: f64, t57: f64, t606: f64, t895: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t3347: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3784, t3787, t3790) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1031(t3768, t3783, t3727, t489, t1204, t1234, t1281, t1285, t1288, t1291, t3552, t3666, t3670, t3746, t3751, t3755, t3756, t3760, t3763, t3767, t3770, t3774, t3778, t3782, t460, t490);
        let t3791 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1032(t1277, t3790);
        let t3794 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1033(t1204, t1210, t1215, t1271, t1274, t1295, t3552, t3556, t3561, t3567, t3569, t3572, t3576, t3585, t3729, t3732, t3739, t3791, t460, t495);
        let (t3798, t3800, t3801) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1034(t1298, t498);
        let t3804 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1035(t1300, t198, t336, t3378, t3381, t3388, t3430, t3438, t3528, t3530, t3533, t3537, t3541, t3545, t3794, t3798, t3801);
        let (t3805, t3812) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1036(t33, t265, t502, t2838, t3804, t1113, t1304, t2258, t3351, t504, t57, t606, t895, dens_threshold, rho1, zeta_threshold);
        let t3813 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1037(t3347, t3812);
    (t3784, t3787, t3790, t3791, t3794, t3798, t3800, t3801, t3805, t3813)
}
