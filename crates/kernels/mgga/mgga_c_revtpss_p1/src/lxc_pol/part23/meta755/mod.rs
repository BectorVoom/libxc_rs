//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta755 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2545;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta755<F: Float>(t53542: F, t3115: F, t42793: F, t4906: F, t3162: F, t999: F, t42865: F, t72: F, t3088: F, t43472: F, t43401: F, t1062: F, t15655: F) -> (F, F, F, F, F, F, F, F) {
        let (t53543, t53613, t53619, t53667, t53668, t53669, t53676, t53692) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2545::<F>(t53542, t3115, t42793, t4906, t3162, t999, t42865, t72, t3088, t43472, t43401, t1062, t15655);
    (t53543, t53613, t53619, t53667, t53668, t53669, t53676, t53692)
}
