//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta433 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1938;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1939;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1940;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1941;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta433<F: Float>(t9858: F, t9861: F, t2619: F, t5635: F, t13664: F, t13667: F, t13669: F, t13671: F, t13673: F, t13682: F, t13683: F, t9524: F, t9542: F, t9588: F, t9854: F, t9865: F, t9868: F, t13881: F, t13882: F, t13884: F, t225: F, t1392: F, t73: F, t13768: F, t3829: F, t1412: F, t5591: F, t1353: F, t3889: F, t5651: F, t13716: F, t1394: F, t1395: F, t1877: F, t1879: F, t4045: F, t4050: F, t4053: F, t539: F, t541: F, t5644: F, t5650: F, t5652: F, t5655: F, t543: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13885, t13886, t13888, t13889) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1938::<F>(t9858, t9861, t2619, t5635, t13664, t13667, t13669, t13671, t13673, t13682, t13683, t9524, t9542, t9588, t9854, t9865, t9868);
        let (t13892, t13902, t13907, t13910, t13911) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1939::<F>(t13881, t13882, t13884, t13889, t225, t1392, t73, t13768, t3829, t1412, t5591, t1353);
        let (t13914, t13917, t13920) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1940::<F>(t3889, t5651, t13716, t1394, t13892, t13902, t13907, t13911, t1392, t1395, t1877, t1879, t4045, t4050, t4053, t539, t541, t5644, t5650, t5652, t5655);
        let t13921 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1941::<F>(t13920, t543);
    (t13885, t13886, t13888, t13892, t13902, t13907, t13910, t13911, t13914, t13917, t13920, t13921)
}
