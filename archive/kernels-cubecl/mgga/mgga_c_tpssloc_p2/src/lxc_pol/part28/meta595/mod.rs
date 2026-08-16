//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta595 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1891;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1892;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta595<F: Float>(t23788: F, t86797: F, t16596: F, t83555: F, t1081: F, t4303: F, t28: F, t40772: F, t86717: F, t25365: F, t1530: F, t3231: F, t1649: F, t2749: F, t57893: F, t2752: F, t13487: F, t1390: F, t16018: F, t26062: F, t645: F, t72: F, t26066: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t89928, t89931, t89941, t89954, t89972, t89978) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1891::<F>(t23788, t86797, t16596, t83555, t1081, t4303, t28, t40772, t86717, t25365, t1530, t3231);
        let (t89982, t89987, t89993, t90023, t90072, t90076) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1892::<F>(t1649, t2749, t23788, t57893, t2752, t13487, t1390, t16018, t26062, t645, t72, t26066);
    (t89928, t89931, t89941, t89954, t89972, t89978, t89982, t89987, t89993, t90023, t90072, t90076)
}
