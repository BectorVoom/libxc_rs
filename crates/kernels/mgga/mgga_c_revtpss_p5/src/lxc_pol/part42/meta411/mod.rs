//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta411 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1441;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1442;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1443;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1444;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta411<F: Float>(t13652: F, t177: F, t6800: F, t762: F, t13666: F, t13668: F, t9858: F, t9861: F, t13887: F, t13664: F, t13682: F, t13683: F, t9524: F, t9542: F, t9588: F, t9854: F, t9865: F, t9868: F, t22190: F, t22203: F, t22210: F, t225: F, t1877: F, t73: F, t4010: F, t6836: F, t1353: F, t5591: F, t5651: F, t1412: F, t6816: F, t1394: F, t21969: F, t1392: F, t1395: F, t1879: F, t539: F, t541: F, t5644: F, t5650: F, t5652: F, t5655: F, t6832: F, t6837: F, t6840: F, t543: F, t1390: F, t828: F, t221: F, t4019: F, t6844: F, t4018: F, t14045: F, t6869: F, t3992: F, t2661: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t22211, t22214, t22215, t22216, t22217, t22218, t22219, t22220) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1441::<F>(t13652, t177, t6800, t762, t13666, t13668, t9858, t9861, t13887, t13664, t13682, t13683, t9524, t9542, t9588, t9854, t9865, t9868);
        let (t22223, t22229, t22237, t22240) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1442::<F>(t22190, t22203, t22210, t22220, t225, t1877, t73, t4010, t6836, t1353, t5591, t5651);
        let t22252 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1443::<F>(t1412, t6816, t1353, t1394, t21969, t1392, t1395, t1877, t1879, t22223, t22229, t22237, t22240, t539, t541, t5644, t5650, t5652, t5655, t6832, t6837, t6840);
        let (t22253, t22255, t22260, t22264) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1444::<F>(t22252, t543, t1390, t828, t221, t4019, t6844, t4018, t14045, t6869, t3992, t2661);
    (t22211, t22214, t22215, t22216, t22217, t22218, t22219, t22253, t22255, t22260, t22264)
}
