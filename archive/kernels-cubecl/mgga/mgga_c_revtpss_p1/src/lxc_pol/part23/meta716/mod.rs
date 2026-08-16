//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta716 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2475;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta716<F: Float>(t5635: F, t9586: F, t5571: F, t9425: F, t9318: F, t1857: F, t9342: F, t9855: F, t9410: F, t9413: F, t9372: F, t13597: F, t2496: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t48280, t48282, t48285, t48287, t48291, t48293, t48294, t48297, t48299) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2475::<F>(t5635, t9586, t5571, t9425, t9318, t1857, t9342, t9855, t9410, t9413, t9372, t13597, t2496);
    (t48280, t48282, t48285, t48287, t48291, t48293, t48294, t48297, t48299)
}
