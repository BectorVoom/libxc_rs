//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta200 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk867;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk868;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta200<F: Float>(t10292: F, t281: F, t283: F, t2403: F, t909: F, t2827: F, t699: F, t2830: F, t2833: F, t241: F, t2978: F, t10216: F, t9288: F, t136: F, t10277: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10294, t10295, t10296, t10298, t10300, t10302, t10304, t10305) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk867::<F>(t10292, t281, t283, t2403, t909, t2827, t699, t2830, t2833, t241, t2978, t10216, t9288);
        let (t10306, t10307, t10309) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk868::<F>(t10304, t10305, t136, t10277, t9288);
    (t10294, t10295, t10296, t10298, t10300, t10302, t10304, t10305, t10306, t10307, t10309)
}
