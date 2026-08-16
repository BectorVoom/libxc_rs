//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta480 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2076;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta480<F: Float>(t16586: F, t2658: F, t2523: F, t5527: F, t262: F, t5544: F, t1484: F, t868: F, t5660: F, t870: F, t12850: F, t12860: F, t16577: F, t16578: F, t16581: F, t16582: F, t16583: F, t2522: F, t4119: F, t4307: F, t4310: F, t4314: F, t776: F, t9457: F, t9469: F, t9476: F, t9484: F, t9496: F) -> (F, F, F, F, F, F) {
        let (t16588, t16589, t16592, t16596, t16606, t16610) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2076::<F>(t16586, t2658, t2523, t5527, t262, t5544, t1484, t868, t5660, t870, t12850, t12860, t16577, t16578, t16581, t16582, t16583, t2522, t4119, t4307, t4310, t4314, t776, t9457, t9469, t9476, t9484, t9496);
    (t16588, t16589, t16592, t16596, t16606, t16610)
}
