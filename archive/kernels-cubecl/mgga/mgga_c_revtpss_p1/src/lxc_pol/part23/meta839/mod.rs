//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta839 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2712;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2713;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta839<F: Float>(t1263: F, t372: F, t6628: F, t21233: F, t3647: F, t17451: F, t17605: F, t17209: F, t17569: F, t20824: F, t3172: F, t3711: F, t20879: F, t1260: F, t20850: F, t11262: F, t3600: F, t6630: F, t17225: F, t5391: F, t21183: F, t20875: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t69839, t69856, t69866, t69885, t69890) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2712::<F>(t1263, t372, t6628, t21233, t3647, t17451, t17605, t17209, t17569, t20824, t3172, t3711);
        let (t69899, t69906, t69910, t69916, t69936, t69939) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2713::<F>(t20879, t3172, t3711, t1260, t20850, t11262, t3600, t6630, t17225, t5391, t21183, t20875);
    (t69839, t69856, t69866, t69885, t69890, t69899, t69906, t69910, t69916, t69936, t69939)
}
