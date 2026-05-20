//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta414 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1887;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1888;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1889;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta414<F: Float>(t1466: F, t2246: F, t1497: F, t2248: F, t4241: F, t644: F, t2315: F, t10355: F, t1469: F, t2251: F, t2275: F, t4186: F, t30: F, t33: F, t606: F, t2258: F, t4201: F, t580: F, t9342: F, zeta_threshold: F, t48: F, t10368: F, t2282: F, t4210: F, t60: F, t10379: F, t1474: F, t1480: F, t2270: F, t2283: F, t2286: F, t4202: F, t4205: F, t44: F, t56: F, t614: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13272, t13283, t13286, t13289, t13299, t13302) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1887::<F>(t1466, t2246, t1497, t2248, t4241, t644, t2315, t10355, t1469, t2251, t2275, t4186);
        let (t13303, t13306, t13312) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1888::<F>(t30, t33, t13302, t606, t2258, t4201, t580, t9342, zeta_threshold);
        let (t13313, t13324, t13334) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1889::<F>(t13312, t48, t10368, t1469, t2251, t2282, t4186, t606, t2258, t4210, t60, t10379, t13299, t13303, t13306, t1474, t1480, t2270, t2283, t2286, t4202, t4205, t44, t56, t614);
    (t13272, t13283, t13286, t13289, t13299, t13302, t13303, t13306, t13312, t13313, t13324, t13334)
}
