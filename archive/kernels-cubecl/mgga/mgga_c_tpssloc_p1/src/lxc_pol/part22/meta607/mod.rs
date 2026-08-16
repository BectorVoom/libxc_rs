//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta607 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2133;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta607<F: Float>(t50361: F, t2770: F, t2987: F, t10277: F, t4509: F, t1606: F, t2402: F, t973: F, t14202: F, t3048: F, t3185: F, t49649: F) -> (F, F, F, F, F, F) {
        let (t50362, t50366, t50370, t50425, t50443, t50465) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2133::<F>(t50361, t2770, t2987, t10277, t4509, t1606, t2402, t973, t14202, t3048, t3185, t49649);
    (t50362, t50366, t50370, t50425, t50443, t50465)
}
