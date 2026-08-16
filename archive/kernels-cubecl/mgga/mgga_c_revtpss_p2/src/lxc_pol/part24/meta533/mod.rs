//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta533 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1571;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1572;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta533<F: Float>(t1882: F, t6843: F, t22881: F, t9962: F, t6869: F, t73856: F, t9816: F, t9818: F, t2661: F, t3992: F, t74026: F, t13999: F, t22843: F, t22854: F, t3989: F, t221: F, t22852: F, t3978: F, t9921: F, t22956: F, t3930: F, t22886: F, t9744: F, t13790: F, t13845: F, t13847: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t85659, t85705, t85735, t85741, t85752) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1571::<F>(t1882, t6843, t22881, t9962, t6869, t73856, t9816, t9818, t2661, t3992, t74026, t13999, t22843);
        let (t85764, t85778, t85782, t85791, t85816) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1572::<F>(t22854, t3989, t221, t22852, t3978, t9921, t22956, t3930, t22886, t9744, t13790, t13845, t13847, t73856);
    (t85659, t85705, t85735, t85741, t85752, t85764, t85778, t85782, t85791, t85816)
}
