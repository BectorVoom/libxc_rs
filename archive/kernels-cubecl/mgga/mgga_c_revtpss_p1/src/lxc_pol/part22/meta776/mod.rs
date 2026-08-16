//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta776 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2865;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2866;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta776<F: Float>(t1235: F, t3661: F, t371: F, t676: F, t1236: F, t2434: F, t3671: F, t3672: F, t12625: F, t458: F, t456: F, t225: F, t43813: F, t12984: F, t3667: F, t1261: F, t12879: F, t247: F, t3372: F, t3368: F, t12881: F, t3647: F, t1224: F, t12268: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t44823, t44829, t44838, t44842, t44843) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2865::<F>(t1235, t3661, t371, t676, t1236, t2434, t3671, t3672, t12625, t458, t456, t225);
        let (t44865, t44884, t44902, t44906, t44917, t44919) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2866::<F>(t43813, t12984, t3667, t1261, t12879, t247, t3372, t3368, t12881, t3647, t1224, t12268);
    (t44823, t44829, t44838, t44842, t44843, t44865, t44884, t44902, t44906, t44917, t44919)
}
