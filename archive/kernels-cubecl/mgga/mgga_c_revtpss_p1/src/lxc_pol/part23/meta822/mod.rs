//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta822 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2673;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2674;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta822<F: Float>(t1065: F, t372: F, t6305: F, t19912: F, t3241: F, t1011: F, t6292: F, t697: F, t11922: F, t19717: F, t4899: F, t11675: F, t19785: F, t15906: F, t19753: F, t20090: F, t3115: F, t19649: F, t11774: F, t20039: F, t53405: F, t19837: F, t19744: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t66187, t66215, t66218, t66221, t66261) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2673::<F>(t1065, t372, t6305, t19912, t3241, t1011, t6292, t697, t11922, t19717, t4899, t11675, t19785);
        let (t66288, t66304, t66306, t66328, t66332, t66355) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2674::<F>(t11922, t15906, t19753, t20090, t3115, t19649, t372, t11774, t20039, t53405, t19837, t19744);
    (t66187, t66215, t66218, t66221, t66261, t66288, t66304, t66306, t66328, t66332, t66355)
}
