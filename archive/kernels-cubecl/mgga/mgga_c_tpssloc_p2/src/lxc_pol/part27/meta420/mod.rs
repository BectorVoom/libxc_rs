//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta420 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1727;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1728;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta420<F: Float>(t1863: F, t22550: F, t6489: F, t9231: F, t1860: F, t1865: F, t22490: F, t22493: F, t22513: F, t22516: F, t22519: F, t22523: F, t22527: F, t22531: F, t22534: F, t22537: F, t22544: F, t22546: F, t22549: F, t6486: F, t6490: F, t6492: F, t6495: F, t6506: F, t6510: F, t5: F, t112: F, t1266: F, t6534: F, t652: F, t192: F, t532: F, t1982: F) -> (F, F, F, F, F, F, F, F) {
        let (t22551, t22554, t22557) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1727::<F>(t1863, t22550, t6489, t9231, t1860, t1865, t22490, t22493, t22513, t22516, t22519, t22523, t22527, t22531, t22534, t22537, t22544, t22546, t22549, t6486, t6490, t6492, t6495, t6506, t6510);
        let (t22558, t22559, t22561, t22563, t22573, t22574) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1728::<F>(t5, t22557, t112, t1266, t6534, t652, t192, t532, t1982);
    (t22551, t22554, t22558, t22559, t22561, t22563, t22573, t22574)
}
