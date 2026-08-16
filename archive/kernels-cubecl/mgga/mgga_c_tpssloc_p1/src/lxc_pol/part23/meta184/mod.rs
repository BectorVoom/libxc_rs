//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta184 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk813;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk814;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta184<F: Float>(t10277: F, t344: F, t241: F, t625: F, t281: F, t283: F, t2978: F, t340: F, t63: F, t221: F, t339: F, t2393: F, t374: F, t376: F, t370: F, t3036: F, t67: F, t3067: F, t3186: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10278, t10292, t10294, t10295, t10304, t10335, t10339, t10375) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk813::<F>(t10277, t344, t241, t625, t281, t283, t2978, t340, t63, t221, t339, t2393, t374, t376);
        let (t10377, t10385, t10401, t10402, t10403) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk814::<F>(t10375, t370, t10335, t221, t339, t3036, t67, t3067, t3186);
    (t10278, t10292, t10294, t10295, t10304, t10339, t10375, t10377, t10385, t10401, t10402, t10403)
}
