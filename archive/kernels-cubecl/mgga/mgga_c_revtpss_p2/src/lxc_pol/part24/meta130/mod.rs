//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta130 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk686;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk687;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk688;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta130<F: Float>(t1469: F, t3362: F, t3367: F, t1130: F, t1719: F, t1723: F, t3390: F, t3407: F, t1729: F, t698: F, t1160: F, t1737: F, t1179: F, t1749: F, t1756: F, t3523: F, t300: F, t3495: F, t1208: F, t1769: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5046, t5051, t5063, t5071, t5087, t5093, t5120) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk686::<F>(t1469, t3362, t3367, t1130, t1719, t1723, t3390, t3407, t1729, t698, t1160, t1737);
        let (t5158, t5184, t5192) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk687::<F>(t1179, t1749, t1756, t3523, t300);
        let (t5197, t5219) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk688::<F>(t1756, t3495, t1208, t1769);
    (t5046, t5051, t5063, t5071, t5087, t5093, t5120, t5158, t5184, t5192, t5197, t5219)
}
