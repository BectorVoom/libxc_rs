//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta725 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2580;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta725<F: Float>(t3400: F, t4832: F, t11282: F, t1687: F, t1682: F, t3357: F, t11310: F, t1694: F, t3401: F, t11420: F, t1098: F, t14956: F) -> (F, F, F, F, F, F, F) {
        let (t51371, t51376, t51382, t51385, t51389, t51392, t51397) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2580::<F>(t3400, t4832, t11282, t1687, t1682, t3357, t11310, t1694, t3401, t11420, t1098, t14956);
    (t51371, t51376, t51382, t51385, t51389, t51392, t51397)
}
