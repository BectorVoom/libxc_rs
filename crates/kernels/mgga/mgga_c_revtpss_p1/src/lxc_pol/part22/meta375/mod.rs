//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta375 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1924;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1925;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1926;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta375<F: Float>(t10275: F, t10278: F, t10284: F, t10287: F, t10295: F, t13261: F, t13262: F, t13263: F, t13264: F, t13265: F, t13266: F, t4171: F, t602: F, t1466: F, t2246: F, t1497: F, t2248: F, t4241: F, t644: F, t2315: F, t10355: F, t1469: F, t2251: F, t2275: F, t4186: F, t30: F, t33: F, t606: F, t2258: F, t4201: F, t580: F, t9342: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13267, t13269) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1924::<F>(t10275, t10278, t10284, t10287, t10295, t13261, t13262, t13263, t13264, t13265, t13266, t4171, t602);
        let (t13272, t13283, t13286, t13289, t13299, t13302) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1925::<F>(t1466, t2246, t1497, t2248, t4241, t644, t2315, t10355, t1469, t2251, t2275, t4186);
        let (t13303, t13306, t13309, t13310, t13312) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1926::<F>(t30, t33, t13302, t606, t2258, t4201, t580, t9342, zeta_threshold);
    (t13267, t13269, t13272, t13283, t13286, t13289, t13299, t13303, t13306, t13309, t13310, t13312)
}
