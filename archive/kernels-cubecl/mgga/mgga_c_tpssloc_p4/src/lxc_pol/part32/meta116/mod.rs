//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta116 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk697;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk698;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta116<F: Float>(t888: F, t892: F, t287: F, t891: F, t275: F, t273: F, t276: F, t2764: F, t241: F, t63: F, t281: F, t283: F, t699: F, t909: F, t976: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2787, t2791, t2792, t2798, t2802, t2810, t2815, t2820, t2822) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk697::<F>(t888, t892, t287, t891, t275, t273, t276, t2764, t241, t63, t281, t283);
        let (t2823, t2824, t2826) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk698::<F>(t2822, t699, t909, t241, t976);
    (t2787, t2791, t2792, t2798, t2802, t2810, t2815, t2820, t2822, t2823, t2824, t2826)
}
