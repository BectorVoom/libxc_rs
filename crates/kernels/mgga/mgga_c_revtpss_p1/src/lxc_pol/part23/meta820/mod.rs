//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta820 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2669;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2670;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta820<F: Float>(t15618: F, t15682: F, t1062: F, t53877: F, t15827: F, t19878: F, t15711: F, t4834: F, t11672: F, t19785: F, t1045: F, t4772: F, t15707: F, t15769: F, t12013: F, t20029: F, t1063: F, t19671: F, t3172: F, t19697: F, t3173: F, t1041: F, t19799: F, t11262: F, t6301: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t65823, t65837, t65840, t65859, t65892, t65894) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2669::<F>(t15618, t15682, t1062, t53877, t15827, t19878, t15711, t4834, t11672, t19785, t1045, t4772);
        let (t65931, t65960, t65965, t66003, t66017, t66022) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2670::<F>(t15707, t15769, t12013, t20029, t1063, t19671, t3172, t19697, t3173, t1041, t19799, t11262, t6301);
    (t65823, t65837, t65840, t65859, t65892, t65894, t65931, t65960, t65965, t66003, t66017, t66022)
}
