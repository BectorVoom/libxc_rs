//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta511 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2027;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2028;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta511<F: Float>(t12458: F, t12461: F, t677: F, t9713: F, t3684: F, t181: F, t2558: F, t686: F, t1291: F, t2369: F, t9720: F, t9843: F, t1294: F, t3814: F, t9874: F, t1307: F, t3914: F, t2411: F, t2414: F, t39246: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t39350, t39354, t39356, t39358, t39360, t39362) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2027::<F>(t12458, t12461, t677, t9713, t3684, t181, t2558, t686, t1291, t2369, t9720, t9843);
        let (t39364, t39365, t39367, t39373) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2028::<F>(t1294, t39362, t3814, t9874, t1307, t3914, t2411, t2414, t39246);
    (t39350, t39354, t39356, t39358, t39360, t39362, t39364, t39365, t39367, t39373)
}
