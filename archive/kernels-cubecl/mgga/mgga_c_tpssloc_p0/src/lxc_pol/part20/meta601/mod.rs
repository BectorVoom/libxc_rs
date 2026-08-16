//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta601 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2181;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta601<F: Float>(t1174: F, t1179: F, t44633: F, t11529: F, t3460: F, t3456: F, t10469: F, t1190: F, t11887: F, t42339: F, t466: F, t11715: F, t42341: F) -> (F, F, F, F, F, F, F) {
        let (t44635, t44638, t44641, t44690, t44691, t44696, t44698) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2181::<F>(t1174, t1179, t44633, t11529, t3460, t3456, t10469, t1190, t11887, t42339, t466, t11715, t42341);
    (t44635, t44638, t44641, t44690, t44691, t44696, t44698)
}
