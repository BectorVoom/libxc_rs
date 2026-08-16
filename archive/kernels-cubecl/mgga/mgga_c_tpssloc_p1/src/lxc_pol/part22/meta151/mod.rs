//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta151 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk955;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk956;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk957;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta151<F: Float>(t1625: F, t990: F, t4343: F, t977: F, t2979: F, t4338: F, t1539: F, t248: F, t3051: F, t1041: F, t1616: F, t884: F, t3071: F, t1023: F, t247: F, t375: F, t1043: F, t2775: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t4559, t4562, t4565, t4571) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk955::<F>(t1625, t990, t4343, t977, t2979, t4338, t1539, t248, t3051);
        let (t4572, t4574, t4575, t4578, t4579, t4582) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk956::<F>(t1041, t4571, t1616, t884, t3071, t1023, t1539, t247, t375);
        let t4583 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk957::<F>(t1043, t2775);
    (t4559, t4562, t4565, t4571, t4572, t4574, t4575, t4578, t4579, t4582, t4583)
}
