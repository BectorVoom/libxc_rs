//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta678 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2561;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2562;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta678<F: Float>(t11629: F, t4869: F, t14967: F, t3411: F, t51474: F, t51476: F, t51478: F, t51480: F, t51482: F, t51485: F, t51549: F, t51593: F, t51831: F, t11366: F, t1164: F, t14853: F, t11129: F, t1694: F, t43689: F, t43692: F, t11400: F, t4874: F, t11365: F, t300: F, t4861: F, t51811: F) -> (F, F, F, F, F, F, F) {
        let (t51833, t51835, t51836) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2561::<F>(t11629, t4869, t14967, t3411, t51474, t51476, t51478, t51480, t51482, t51485, t51549, t51593, t51831);
        let (t51839, t51844, t51847, t51851) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2562::<F>(t11366, t1164, t14853, t11129, t1694, t43689, t43692, t11400, t4874, t11365, t300, t4861, t51811);
    (t51833, t51835, t51836, t51839, t51844, t51847, t51851)
}
