//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta466 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2147;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2148;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta466<F: Float>(t11703: F, t15595: F, t11678: F, t357: F, t1592: F, t3092: F, t4900: F, t999: F, t4893: F, t3117: F, t4894: F) -> (F, F, F, F, F, F, F, F) {
        let (t15596, t15599, t15600, t15601, t15604) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2147::<F>(t11703, t15595, t11678, t357, t1592, t3092, t4900, t999);
        let (t15605, t15606, t15609) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2148::<F>(t15604, t4893, t3117, t4894, t999);
    (t15596, t15599, t15600, t15601, t15604, t15605, t15606, t15609)
}
