//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta220 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1267;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta220<F: Float>(t334: F, t371: F, t533: F, t556: F, t1184: F, t460: F, t1433: F, t71: F, t1458: F, t89: F, t1597: F, t343: F) -> (F, F, F, F, F, F) {
        let (t6793, t6924, t7319, t7445, t7458, t7577) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1267::<F>(t334, t371, t533, t556, t1184, t460, t1433, t71, t1458, t89, t1597, t343);
    (t6793, t6924, t7319, t7445, t7458, t7577)
}
