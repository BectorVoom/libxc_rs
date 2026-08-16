//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta414 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1232;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta414<F: Float>(t184: F, t20217: F, t120: F, t20856: F, t46657: F, t5593: F, t20852: F, t13258: F, t20983: F, t20974: F, t9638: F, t20891: F) -> (F, F, F, F, F, F, F) {
        let (t67469, t67607, t67612, t67620, t67625, t67637, t67639) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1232::<F>(t184, t20217, t120, t20856, t46657, t5593, t20852, t13258, t20983, t20974, t9638, t20891);
    (t67469, t67607, t67612, t67620, t67625, t67637, t67639)
}
