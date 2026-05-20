//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta759 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2552;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2553;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta759<F: Float>(t43347: F, t53668: F, t11852: F, t41270: F, t3316: F, t4746: F, t4891: F, t16381: F, t3090: F, t11262: F, t3127: F, t4874: F, t15749: F, t3211: F, t16199: F, t372: F, t16208: F, t1025: F, t1663: F, t2434: F, t371: F, t225: F, t53166: F, t366: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t54509, t54537, t54570, t54578, t54599) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2552::<F>(t43347, t53668, t11852, t41270, t3316, t4746, t4891, t16381, t3090, t11262, t3127, t4874);
        let (t54648, t54658, t54672, t54687, t54695, t54696) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2553::<F>(t15749, t3211, t16199, t372, t16208, t1025, t1663, t2434, t371, t225, t53166, t366);
    (t54509, t54537, t54570, t54578, t54599, t54648, t54658, t54672, t54687, t54695, t54696)
}
