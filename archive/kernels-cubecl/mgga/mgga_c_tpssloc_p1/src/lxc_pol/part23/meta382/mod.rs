//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta382 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1184;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta382<F: Float>(t1667: F, t9709: F, t11274: F, t1657: F, t11189: F, t11282: F, t1687: F, t11419: F, t1675: F, t11349: F, t11292: F, t1714: F, t44583: F) -> (F, F, F, F, F, F, F, F) {
        let (t50846, t51120, t51249, t51376, t51427, t51604, t51680, t51968) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1184::<F>(t1667, t9709, t11274, t1657, t11189, t11282, t1687, t11419, t1675, t11349, t11292, t1714, t44583);
    (t50846, t51120, t51249, t51376, t51427, t51604, t51680, t51968)
}
