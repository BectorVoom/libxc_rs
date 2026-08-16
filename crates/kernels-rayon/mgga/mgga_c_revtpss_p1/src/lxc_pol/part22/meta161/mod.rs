//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta161 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1072;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1073;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1074;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1075;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1076;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1077;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1078;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta161(t1298: f64, t498: f64, t1300: f64, t198: f64, t336: f64, t3378: f64, t3381: f64, t3388: f64, t3430: f64, t3438: f64, t3528: f64, t3530: f64, t3533: f64, t3537: f64, t3541: f64, t3545: f64, t3794: f64, t33: f64, t265: f64, t502: f64, t2838: f64, t1113: f64, t1304: f64, t2258: f64, t3351: f64, t504: f64, t57: f64, t606: f64, t895: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t3347: f64, t1312: f64, t2320: f64, t2322: f64, t2327: f64, t2371: f64, t670: f64, t93: f64, t1330: f64, t72: f64, t757: f64, t530: f64, t566: f64, t1353: f64, t525: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3798, t3800, t3801) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1072(t1298, t498);
        let t3804 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1073(t1300, t198, t336, t3378, t3381, t3388, t3430, t3438, t3528, t3530, t3533, t3537, t3541, t3545, t3794, t3798, t3801);
        let (t3805, t3812) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1074(t33, t265, t502, t2838, t3804, t1113, t1304, t2258, t3351, t504, t57, t606, t895, dens_threshold, rho1, zeta_threshold);
        let t3813 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1075(t3347, t3812);
        let (t3821, t3825, t3826, t3827, t3828) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1076(t1312, t2320, t2322, t2327, t2371, t670, t93, t1330, t72, t757, t530, t566);
        let t3829 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1077(t1353);
        let t3833 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1078(t525);
    (t3798, t3800, t3801, t3805, t3813, t3821, t3825, t3826, t3827, t3828, t3829, t3833)
}
