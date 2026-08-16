//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta281 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk975;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta281<F: Float>(t20852: F, t232: F, t860: F, t1509: F, t5584: F, t9975: F, t10080: F, t2632: F, t2728: F, t13416: F, t5585: F, t1510: F, t17030: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t20853, t20854, t20856, t20857, t20858, t20861, t20862, t20867, t20870, t20871, t20873) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk975::<F>(t20852, t232, t860, t1509, t5584, t9975, t10080, t2632, t2728, t13416, t5585, t1510, t17030);
    (t20853, t20854, t20856, t20857, t20858, t20861, t20862, t20867, t20870, t20871, t20873)
}
