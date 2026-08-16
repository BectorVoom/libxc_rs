//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta223 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk926;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk927;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta223<F: Float>(t10658: F, t10856: F, t360: F, t1021: F, t248: F, t1004: F, t3047: F, t3053: F, t3117: F, t1043: F, t676: F, t884: F, t1041: F, t3048: F, t10478: F, t3128: F, t10472: F, t10481: F, t3131: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10857, t10858, t10860, t10863) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk926::<F>(t10658, t10856, t360, t1021, t248, t1004, t3047);
        let (t10866, t10868, t10870, t10871, t10873, t10875, t10876, t10877) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk927::<F>(t3053, t3117, t1043, t676, t248, t884, t1041, t3048, t10478, t3128, t10472, t10481, t3131);
    (t10857, t10858, t10860, t10863, t10866, t10868, t10870, t10871, t10873, t10875, t10876, t10877)
}
