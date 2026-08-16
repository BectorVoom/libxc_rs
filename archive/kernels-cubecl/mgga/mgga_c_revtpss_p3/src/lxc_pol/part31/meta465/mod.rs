//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta465 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1706;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1707;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1708;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1709;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta465<F: Float>(t22046: F, t3936: F, t3938: F, t5659: F, t5673: F, t5674: F, t1399: F, t125: F, t6836: F, t9955: F, t1413: F, t6816: F, t547: F, t807: F, t4011: F, t1353: F, t6883: F, t800: F, t13832: F, t13851: F, t13858: F, t3934: F, t3944: F, t9739: F, t9742: F, t9766: F, t13784: F, t13790: F, t13789: F, t13880: F, t13943: F, t13949: F, t13954: F, t13956: F, t5671: F, t9776: F, t9780: F, t9786: F, t9791: F, t9796: F, t9799: F, t6871: F, t9962: F, t22016: F, t5675: F, t6849: F, t1872: F, t5591: F, t13804: F, t13959: F, t13987: F, t13988: F, t14001: F, t14007: F, t9748: F, t9804: F, t9847: F, t9910: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t22107, t22111, t22115, t22120, t22125) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1706::<F>(t22046, t3936, t3938, t5659, t5673, t5674, t1399, t125, t6836, t9955, t1413, t6816);
        let (t22129, t22140) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1707::<F>(t22125, t547, t807, t4011, t6836, t1353, t6883, t800, t13832, t13851, t13858, t22107, t22111, t22115, t22120, t3934, t3944, t9739, t9742, t9766);
        let (t22146, t22153) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1708::<F>(t13784, t13790, t13789, t13880, t13943, t13949, t13954, t13956, t5671, t9776, t9780, t9786, t9791, t9796, t9799);
        let (t22159, t22163, t22176) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1709::<F>(t6871, t9962, t22016, t22046, t5673, t5675, t1353, t6849, t800, t1872, t5591, t13804, t13959, t13987, t13988, t14001, t14007, t3944, t5671, t9748, t9804, t9847, t9910);
    (t22107, t22111, t22115, t22120, t22125, t22129, t22140, t22146, t22153, t22159, t22163, t22176)
}
