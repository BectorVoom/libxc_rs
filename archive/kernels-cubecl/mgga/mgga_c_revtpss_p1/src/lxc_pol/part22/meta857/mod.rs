//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta857 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3003;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3004;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta857<F: Float>(t14701: F, t40731: F, t14468: F, t221: F, t2674: F, t2675: F, t14662: F, t231: F, t243: F, t2661: F, t2662: F, t14648: F, t14832: F, t2430: F, t10777: F, t10779: F, t14671: F, t14872: F, t10811: F, t14682: F, t14804: F, t14923: F, t4457: F, t837: F, t14853: F, t2652: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t50298, t50303, t50308, t50312) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3003::<F>(t14701, t40731, t14468, t221, t2674, t2675, t14662, t231, t243, t2661, t2662, t14648, t14832, t2430);
        let (t50325, t50328, t50347, t50351, t50353) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3004::<F>(t10777, t10779, t14671, t14872, t10811, t14682, t14804, t14923, t4457, t837, t14853, t2652);
    (t50298, t50303, t50308, t50312, t50325, t50328, t50347, t50351, t50353)
}
