//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta741 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2806;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2807;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta741<F: Float>(t10073: F, t10654: F, t10959: F, t2439: F, t2777: F, t10914: F, t2710: F, t9285: F, t10972: F, t2470: F, t874: F, t136: F, t2457: F, t2760: F, t10929: F, t10069: F, t2790: F, t9292: F, t11003: F, t9303: F, t10981: F, t22: F, t868: F, t886: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t40924, t40938, t40945, t40948, t40952) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2806::<F>(t10073, t10654, t10959, t2439, t2777, t10914, t2710, t9285, t10972, t2470, t874, t136, t2457, t2760);
        let (t40954, t40956, t40958, t40970, t40978) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2807::<F>(t10073, t10929, t10069, t10654, t2790, t9292, t11003, t9303, t10981, t22, t868, t886);
    (t40924, t40938, t40945, t40948, t40952, t40954, t40956, t40958, t40970, t40978)
}
