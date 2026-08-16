//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta860 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3009;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3010;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta860<F: Float>(t14786: F, t231: F, t40834: F, t854: F, t14833: F, t236: F, t2453: F, t9794: F, t125: F, t14662: F, t10777: F, t14671: F, t14917: F, t40725: F, t10811: F, t14678: F, t10871: F, t1558: F, t10726: F, t10943: F, t2661: F, t4352: F, t14547: F, t40693: F, t2475: F, t2662: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t50451, t50453, t50457, t50459, t50466) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3009::<F>(t14786, t231, t40834, t854, t14833, t236, t2453, t9794, t125, t14662, t10777, t14671, t14917, t40725);
        let (t50472, t50474, t50493, t50497, t50502) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3010::<F>(t10811, t14678, t10871, t1558, t10726, t10943, t2661, t4352, t14547, t40693, t14917, t2475, t2662);
    (t50451, t50453, t50457, t50459, t50466, t50472, t50474, t50493, t50497, t50502)
}
