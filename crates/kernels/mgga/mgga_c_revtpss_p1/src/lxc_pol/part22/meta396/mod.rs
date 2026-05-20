//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta396 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1977;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1978;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1979;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1980;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1981;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta396<F: Float>(t13620: F, t13622: F, t13623: F, t13624: F, t13629: F, t13631: F, t13633: F, t13634: F, t13635: F, t13636: F, t13637: F, t9394: F, t9415: F, t9421: F, t9427: F, t9546: F, t13640: F, t13641: F, t13643: F, t13644: F, t13645: F, t13646: F, t13647: F, t13653: F, t13655: F, t9514: F, t9517: F, t9521: F, t9555: F, t9569: F, t9574: F, t9577: F, t9858: F, t9861: F, t2619: F, t5635: F, t13664: F, t13667: F, t13669: F, t13671: F, t13673: F, t13682: F, t13683: F, t9524: F, t9542: F, t9588: F, t9854: F, t9865: F, t9868: F, t13881: F, t225: F, t1392: F, t73: F, t13768: F, t3829: F, t1412: F, t5591: F, t1353: F, t3889: F, t5651: F, t13716: F, t1394: F, t1395: F, t1877: F, t1879: F, t4045: F, t4050: F, t4053: F, t539: F, t541: F, t5644: F, t5650: F, t5652: F, t5655: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t13882 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1977::<F>(t13620, t13622, t13623, t13624, t13629, t13631, t13633, t13634, t13635, t13636, t13637, t9394, t9415, t9421, t9427, t9546);
        let t13884 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1978::<F>(t13640, t13641, t13643, t13644, t13645, t13646, t13647, t13653, t13655, t9514, t9517, t9521, t9555, t9569, t9574, t9577);
        let (t13885, t13886, t13887, t13888, t13889) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1979::<F>(t9858, t9861, t2619, t5635, t13664, t13667, t13669, t13671, t13673, t13682, t13683, t9524, t9542, t9588, t9854, t9865, t9868);
        let (t13892, t13902, t13907, t13910, t13911) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1980::<F>(t13881, t13882, t13884, t13889, t225, t1392, t73, t13768, t3829, t1412, t5591, t1353);
        let (t13914, t13917, t13920) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1981::<F>(t3889, t5651, t13716, t1394, t13892, t13902, t13907, t13911, t1392, t1395, t1877, t1879, t4045, t4050, t4053, t539, t541, t5644, t5650, t5652, t5655);
    (t13885, t13886, t13887, t13888, t13892, t13902, t13907, t13910, t13911, t13914, t13917, t13920)
}
