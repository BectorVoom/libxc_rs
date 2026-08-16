//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta548 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2046;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2047;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta548<F: Float>(t116: F, t1314: F, t9534: F, t1307: F, t133: F, t6600: F, t59: F, t9223: F, t120: F, t212: F, t22815: F, t67: F, t535: F, t1317: F, t40005: F, t9580: F, t3741: F, t2566: F, t3732: F, t12214: F, t792: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t40369, t40372, t40394, t40399) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2046::<F>(t116, t1314, t9534, t1307, t133, t6600, t59, t9223, t120, t212, t22815, t67);
        let (t40401, t40402, t40406, t40407, t40409, t40412) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2047::<F>(t40394, t40399, t535, t1317, t40005, t1314, t9580, t3741, t2566, t3732, t12214, t792);
    (t40369, t40372, t40394, t40399, t40401, t40402, t40406, t40407, t40409, t40412)
}
