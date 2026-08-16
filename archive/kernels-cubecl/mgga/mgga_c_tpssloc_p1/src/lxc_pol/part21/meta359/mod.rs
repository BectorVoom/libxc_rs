//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta359 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1776;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1777;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta359<F: Float>(t2633: F, t4180: F, t4181: F, t13029: F, t225: F, t237: F, t2697: F, t4261: F, t12971: F, t820: F, t847: F, t9645: F, t1484: F, t828: F, t2647: F, t1516: F, t9993: F, t2696: F, t4166: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13333, t13336, t13337, t13345, t13347, t13350) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1776::<F>(t2633, t4180, t4181, t13029, t225, t237, t2697, t4261, t12971, t820, t847, t9645);
        let (t13351, t13352, t13353, t13359, t13360) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1777::<F>(t1484, t828, t2647, t13350, t1516, t9993, t2696, t4166);
    (t13333, t13336, t13337, t13345, t13347, t13350, t13351, t13352, t13353, t13359, t13360)
}
