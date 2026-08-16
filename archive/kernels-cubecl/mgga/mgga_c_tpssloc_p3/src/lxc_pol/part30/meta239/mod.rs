//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta239 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1073;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1074;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1075;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1076;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1077;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1078;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta239<F: Float>(t28: F, t265: F, t504: F, t5669: F, t6278: F, t1409: F, t1534: F, t1649: F, t1768: F, t506: F, t52: F, t5398: F, t5966: F, dens_threshold: F, rho1: F, zeta_threshold: F, t5962: F, t1268: F, t1458: F, t4028: F, t5450: F, t5456: F, t5493: F, t88: F, t5155: F, t5158: F, t1799: F, t5122: F, t5169: F, t25: F, t1408: F, t3664: F, t514: F, t5397: F, t3672: F, t517: F, t157: F, t182: F, t2408: F, t2417: F, t2423: F, t2426: F, t3686: F, t3688: F, t3690: F, t3695: F, t3813: F, t3918: F, t1845: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6279, t6286) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1073::<F>(t28, t265, t504, t5669, t6278, t1409, t1534, t1649, t1768, t506, t52, t5398, t5966, dens_threshold, rho1, zeta_threshold);
        let t6287 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1074::<F>(t5962, t6286);
        let (t6295, t6299, t6300, t6301, t6304) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1075::<F>(t1268, t1458, t4028, t5450, t5456, t5493, t88, t5155, t5158, t1799, t5122, t5169);
        let (t6305, t6312, t6320) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1076::<F>(t25, t28, t1408, t3664, t514, t5397, t1649, t3672, t517, t5966, t157, zeta_threshold);
        let (t6322, t6323) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1077::<F>(t182, t6320, t2408, t2417, t2423, t2426, t3686, t3688, t3690, t3695, t3813, t3918, t6299, t6300, t6301, t6304);
        let t6324 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1078::<F>(t1845);
    (t6279, t6287, t6295, t6299, t6300, t6304, t6305, t6312, t6320, t6322, t6323, t6324)
}
