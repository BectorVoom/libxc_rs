//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta242 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1336;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta242<F: Float>(t10277: F, t344: F, t698: F, t986: F, t973: F, t241: F, t625: F, t281: F, t283: F, t2403: F, t909: F, t2978: F) -> (F, F, F, F, F, F, F) {
        let (t10278, t10287, t10292, t10294, t10295, t10296, t10304) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1336::<F>(t10277, t344, t698, t986, t973, t241, t625, t281, t283, t2403, t909, t2978);
    (t10278, t10287, t10292, t10294, t10295, t10296, t10304)
}
