//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta448 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1293;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1294;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta448<F: Float>(t20305: F, t626: F, t20308: F, t20343: F, t1858: F, t6470: F, t1851: F, t6483: F, t22453: F, t576: F, t112: F, t22430: F, t5392: F) -> (F, F, F, F, F, F, F, F) {
        let (t75592, t75601, t75613, t75768, t75774, t75780, t75784) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1293::<F>(t20305, t626, t20308, t20343, t1858, t6470, t1851, t6483, t22453, t576, t112, t22430);
        let t75836 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1294::<F>(t5392);
    (t75592, t75601, t75613, t75768, t75774, t75780, t75784, t75836)
}
