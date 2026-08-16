//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta567 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2275;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta567<F: Float>(t11883: F, t1215: F, t6252: F, t1751: F, t5011: F, t1246: F, t6238: F, t19145: F, t3612: F, t1734: F, t5052: F, t1235: F, t6218: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t19165, t19166, t19169, t19170, t19174, t19176, t19179, t19180, t19189) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2275::<F>(t11883, t1215, t6252, t1751, t5011, t1246, t6238, t19145, t3612, t1734, t5052, t1235, t6218);
    (t19165, t19166, t19169, t19170, t19174, t19176, t19179, t19180, t19189)
}
