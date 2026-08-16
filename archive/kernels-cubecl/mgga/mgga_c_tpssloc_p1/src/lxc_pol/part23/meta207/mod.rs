//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta207 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk849;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk850;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta207<F: Float>(t12214: F, t205: F, t116: F, t547: F, t535: F, t9534: F, t9538: F, t1337: F, t562: F, t3792: F, t550: F, t1339: F, t836: F, t1336: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t12215, t12225, t12236, t12247, t12248, t12249, t12250, t12282) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk849::<F>(t12214, t205, t116, t547, t535, t9534, t9538, t1337, t562, t3792, t550, t1339, t836);
        let t12283 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk850::<F>(t12282, t1336);
    (t12215, t12225, t12236, t12247, t12248, t12249, t12250, t12282, t12283)
}
