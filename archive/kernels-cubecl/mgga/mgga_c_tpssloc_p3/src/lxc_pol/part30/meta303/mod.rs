//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta303 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1322;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1323;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta303<F: Float>(t10292: F, t281: F, t283: F, t2403: F, t909: F, t241: F, t2978: F, t2967: F, t964: F, t340: F, t63: F, t344: F, t221: F, t339: F, t1032: F, t3082: F, t2393: F, t374: F, t376: F, t370: F, t3158: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10294, t10295, t10296, t10304, t10333, t10335, t10336) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1322::<F>(t10292, t281, t283, t2403, t909, t241, t2978, t2967, t964, t340, t63, t344);
        let (t10339, t10372, t10375, t10377, t10381, t10383) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1323::<F>(t10336, t221, t339, t1032, t3082, t2393, t374, t376, t370, t3158, t964, t10335);
    (t10294, t10295, t10296, t10304, t10333, t10336, t10339, t10372, t10375, t10377, t10381, t10383)
}
