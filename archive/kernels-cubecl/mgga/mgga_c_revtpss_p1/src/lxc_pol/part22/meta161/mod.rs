//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta161 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1072;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1073;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1074;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1075;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1076;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1077;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1078;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta161<F: Float>(t1298: F, t498: F, t1300: F, t198: F, t336: F, t3378: F, t3381: F, t3388: F, t3430: F, t3438: F, t3528: F, t3530: F, t3533: F, t3537: F, t3541: F, t3545: F, t3794: F, t33: F, t265: F, t502: F, t2838: F, t1113: F, t1304: F, t2258: F, t3351: F, t504: F, t57: F, t606: F, t895: F, dens_threshold: F, rho1: F, zeta_threshold: F, t3347: F, t1312: F, t2320: F, t2322: F, t2327: F, t2371: F, t670: F, t93: F, t1330: F, t72: F, t757: F, t530: F, t566: F, t1353: F, t525: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3798, t3800, t3801) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1072::<F>(t1298, t498);
        let t3804 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1073::<F>(t1300, t198, t336, t3378, t3381, t3388, t3430, t3438, t3528, t3530, t3533, t3537, t3541, t3545, t3794, t3798, t3801);
        let (t3805, t3812) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1074::<F>(t33, t265, t502, t2838, t3804, t1113, t1304, t2258, t3351, t504, t57, t606, t895, dens_threshold, rho1, zeta_threshold);
        let t3813 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1075::<F>(t3347, t3812);
        let (t3821, t3825, t3826, t3827, t3828) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1076::<F>(t1312, t2320, t2322, t2327, t2371, t670, t93, t1330, t72, t757, t530, t566);
        let t3829 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1077::<F>(t1353);
        let t3833 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1078::<F>(t525);
    (t3798, t3800, t3801, t3805, t3813, t3821, t3825, t3826, t3827, t3828, t3829, t3833)
}
