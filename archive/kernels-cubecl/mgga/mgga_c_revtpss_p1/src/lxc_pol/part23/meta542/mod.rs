//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta542 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2091;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2092;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta542<F: Float>(t22125: F, t547: F, t807: F, t4011: F, t6836: F, t1353: F, t6883: F, t800: F, t13832: F, t13851: F, t13858: F, t22107: F, t22111: F, t22115: F, t22120: F, t3934: F, t3944: F, t9739: F, t9742: F, t9766: F, t13784: F, t13790: F, t13789: F, t13880: F, t13943: F, t13949: F, t13954: F, t13956: F, t5671: F, t9776: F, t9780: F, t9786: F, t9791: F, t9796: F, t9799: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t22126, t22127, t22129, t22130, t22131, t22135, t22140) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2091::<F>(t22125, t547, t807, t4011, t6836, t1353, t6883, t800, t13832, t13851, t13858, t22107, t22111, t22115, t22120, t3934, t3944, t9739, t9742, t9766);
        let (t22145, t22146, t22153) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2092::<F>(t13784, t13790, t13789, t13880, t13943, t13949, t13954, t13956, t5671, t9776, t9780, t9786, t9791, t9796, t9799);
    (t22126, t22127, t22129, t22130, t22131, t22135, t22140, t22145, t22146, t22153)
}
