//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta284 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1576;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1577;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta284<F: Float>(t698: F, t986: F, t973: F, t135: F, t3010: F, t241: F, t625: F, t281: F, t283: F, t2403: F, t909: F, t2827: F, t699: F, t2830: F, t2833: F, t2978: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10286, t10287, t10290, t10292, t10294, t10295, t10296) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1576::<F>(t698, t986, t973, t135, t3010, t241, t625, t281, t283, t2403, t909);
        let (t10298, t10300, t10302, t10304) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1577::<F>(t2827, t699, t2830, t2833, t241, t2978);
    (t10286, t10287, t10290, t10292, t10294, t10295, t10296, t10298, t10300, t10302, t10304)
}
