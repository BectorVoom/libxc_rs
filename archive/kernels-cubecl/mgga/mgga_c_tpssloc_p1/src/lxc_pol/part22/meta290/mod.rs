//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta290 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1445;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1446;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta290<F: Float>(t68: F, t9971: F, t226: F, t1519: F, t2627: F, t4265: F, t814: F, t4280: F, t808: F, t225: F, t4149: F, t4351: F, t892: F, t1543: F, t2841: F) -> (F, F, F, F, F, F, F, F) {
        let (t13396, t13397, t13416, t13433, t13453, t13463, t13515) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1445::<F>(t68, t9971, t226, t1519, t2627, t4265, t814, t4280, t808, t225, t4149, t4351, t892);
        let t13520 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1446::<F>(t1543, t2841);
    (t13396, t13397, t13416, t13433, t13453, t13463, t13515, t13520)
}
