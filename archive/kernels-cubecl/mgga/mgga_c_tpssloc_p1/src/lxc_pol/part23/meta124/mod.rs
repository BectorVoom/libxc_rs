//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta124 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk620;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk621;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk622;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta124<F: Float>(t1340: F, t5234: F, t1358: F, t1815: F, t1362: F, t242: F, t3788: F, t1336: F, t557: F, t67: F, t246: F, t120: F, t1824: F) -> (F, F, F, F, F, F, F) {
        let (t5235, t5238, t5240) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk620::<F>(t1340, t5234, t1358, t1815, t1362);
        let (t5245, t5246, t5248) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk621::<F>(t242, t3788, t1336, t557, t67, t246);
        let t5249 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk622::<F>(t120, t1824);
    (t5235, t5238, t5240, t5245, t5246, t5248, t5249)
}
