//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta403 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1213;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1214;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta403<F: Float>(t13965: F, t4641: F, t1020: F, t10508: F, t248: F, t5867: F, t3039: F, t5878: F, t14202: F, t4644: F, t3082: F, t5905: F, t1041: F, t43338: F, t5677: F, t3070: F, t43198: F, t5908: F, t5884: F, t698: F, t973: F, t5889: F, t5893: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t62148, t62177, t62183, t62284, t62360) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1213::<F>(t13965, t4641, t1020, t10508, t248, t5867, t3039, t5878, t14202, t4644, t3082, t5905);
        let (t62445, t62494, t62559, t62565, t62832) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1214::<F>(t1041, t248, t43338, t5677, t3070, t43198, t5908, t5884, t698, t973, t5889, t5893);
    (t62148, t62177, t62183, t62284, t62360, t62445, t62494, t62559, t62565, t62832)
}
