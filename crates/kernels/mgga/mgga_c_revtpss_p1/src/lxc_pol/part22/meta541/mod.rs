//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta541 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2352;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta541<F: Float>(t17376: F, t3599: F, t17482: F, t3604: F, t3720: F, t3372: F, t5277: F, t1042: F, t12855: F, t12964: F, t12979: F, t12985: F, t12996: F, t17569: F, t3606: F, t3620: F, t3640: F, t3711: F, t3714: F, t5381: F, t5391: F) -> (F, F, F, F, F, F) {
        let (t17572, t17579, t17580, t17583, t17584, t17587) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2352::<F>(t17376, t3599, t17482, t3604, t3720, t3372, t5277, t1042, t12855, t12964, t12979, t12985, t12996, t17569, t3606, t3620, t3640, t3711, t3714, t5381, t5391);
    (t17572, t17579, t17580, t17583, t17584, t17587)
}
