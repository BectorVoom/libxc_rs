//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta433 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1938;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1939;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1940;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1941;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta433(t9858: f64, t9861: f64, t2619: f64, t5635: f64, t13664: f64, t13667: f64, t13669: f64, t13671: f64, t13673: f64, t13682: f64, t13683: f64, t9524: f64, t9542: f64, t9588: f64, t9854: f64, t9865: f64, t9868: f64, t13881: f64, t13882: f64, t13884: f64, t225: f64, t1392: f64, t73: f64, t13768: f64, t3829: f64, t1412: f64, t5591: f64, t1353: f64, t3889: f64, t5651: f64, t13716: f64, t1394: f64, t1395: f64, t1877: f64, t1879: f64, t4045: f64, t4050: f64, t4053: f64, t539: f64, t541: f64, t5644: f64, t5650: f64, t5652: f64, t5655: f64, t543: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13885, t13886, t13888, t13889) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1938(t9858, t9861, t2619, t5635, t13664, t13667, t13669, t13671, t13673, t13682, t13683, t9524, t9542, t9588, t9854, t9865, t9868);
        let (t13892, t13902, t13907, t13910, t13911) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1939(t13881, t13882, t13884, t13889, t225, t1392, t73, t13768, t3829, t1412, t5591, t1353);
        let (t13914, t13917, t13920) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1940(t3889, t5651, t13716, t1394, t13892, t13902, t13907, t13911, t1392, t1395, t1877, t1879, t4045, t4050, t4053, t539, t541, t5644, t5650, t5652, t5655);
        let t13921 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1941(t13920, t543);
    (t13885, t13886, t13888, t13892, t13902, t13907, t13910, t13911, t13914, t13917, t13920, t13921)
}
