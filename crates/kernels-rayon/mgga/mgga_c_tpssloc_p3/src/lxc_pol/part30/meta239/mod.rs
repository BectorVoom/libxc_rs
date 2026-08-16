//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta239 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1073;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1074;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1075;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1076;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1077;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1078;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta239(t28: f64, t265: f64, t504: f64, t5669: f64, t6278: f64, t1409: f64, t1534: f64, t1649: f64, t1768: f64, t506: f64, t52: f64, t5398: f64, t5966: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t5962: f64, t1268: f64, t1458: f64, t4028: f64, t5450: f64, t5456: f64, t5493: f64, t88: f64, t5155: f64, t5158: f64, t1799: f64, t5122: f64, t5169: f64, t25: f64, t1408: f64, t3664: f64, t514: f64, t5397: f64, t3672: f64, t517: f64, t157: f64, t182: f64, t2408: f64, t2417: f64, t2423: f64, t2426: f64, t3686: f64, t3688: f64, t3690: f64, t3695: f64, t3813: f64, t3918: f64, t1845: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6279, t6286) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1073(t28, t265, t504, t5669, t6278, t1409, t1534, t1649, t1768, t506, t52, t5398, t5966, dens_threshold, rho1, zeta_threshold);
        let t6287 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1074(t5962, t6286);
        let (t6295, t6299, t6300, t6301, t6304) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1075(t1268, t1458, t4028, t5450, t5456, t5493, t88, t5155, t5158, t1799, t5122, t5169);
        let (t6305, t6312, t6320) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1076(t25, t28, t1408, t3664, t514, t5397, t1649, t3672, t517, t5966, t157, zeta_threshold);
        let (t6322, t6323) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1077(t182, t6320, t2408, t2417, t2423, t2426, t3686, t3688, t3690, t3695, t3813, t3918, t6299, t6300, t6301, t6304);
        let t6324 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1078(t1845);
    (t6279, t6287, t6295, t6299, t6300, t6304, t6305, t6312, t6320, t6322, t6323, t6324)
}
