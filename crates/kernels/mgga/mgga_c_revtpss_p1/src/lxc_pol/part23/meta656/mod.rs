//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta656 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2384;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2385;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta656<F: Float>(t10759: F, t2735: F, t10293: F, t240: F, t243: F, t813: F, t816: F, t798: F, t9726: F, t802: F, t10899: F, t794: F, t159: F, t216: F, t2475: F, t123: F, t212: F, t9291: F, t2786: F, t10914: F, t2710: F, t9285: F, t2790: F, t9292: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t40834, t40846, t40850, t40861, t40862, t40864) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2384::<F>(t10759, t2735, t10293, t240, t243, t813, t816, t798, t9726, t802, t10899, t794);
        let (t40868, t40921, t40922, t40945, t40958) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2385::<F>(t159, t216, t2475, t123, t212, t9291, t2786, t10914, t2710, t9285, t2790, t9292);
    (t40834, t40846, t40850, t40861, t40862, t40864, t40868, t40921, t40922, t40945, t40958)
}
