//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta670 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2472;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2473;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta670<F: Float>(t11791: F, t3490: F, t11789: F, t1227: F, t248: F, t3252: F, t3248: F, t11877: F, t3576: F, t11647: F, t1203: F, t204: F, t486: F, t1213: F, t1216: F, t11716: F, t44833: F, t44834: F, t3503: F, t1174: F, t1197: F, t2402: F, t3584: F, t676: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t44968, t44972, t44976, t44996, t45002, t45017) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2472::<F>(t11791, t3490, t11789, t1227, t248, t3252, t3248, t11877, t3576, t11647, t1203, t204, t486);
        let (t45020, t45030, t45037, t45044, t45046) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2473::<F>(t1213, t1216, t248, t45017, t11716, t44833, t44834, t3503, t1174, t1197, t2402, t3584, t676);
    (t44968, t44972, t44976, t44996, t45002, t45017, t45020, t45030, t45037, t45044, t45046)
}
