//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta530 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2065;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta530<F: Float>(t1307: F, t3850: F, t12392: F, t3799: F, t39037: F, t522: F, t2221: F, t3826: F, t12132: F, t592: F, t3696: F, t1336: F, t1339: F, t2691: F) -> (F, F, F, F, F, F, F) {
        let (t40197, t40206, t40224, t40225, t40230, t40231, t40281) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2065::<F>(t1307, t3850, t12392, t3799, t39037, t522, t2221, t3826, t12132, t592, t3696, t1336, t1339, t2691);
    (t40197, t40206, t40224, t40225, t40230, t40231, t40281)
}
