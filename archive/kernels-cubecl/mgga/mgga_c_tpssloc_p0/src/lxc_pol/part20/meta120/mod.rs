//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta120 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk794;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk795;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta120<F: Float>(t2936: F, t300: F, t2898: F, t938: F, t961: F, t2904: F, t2906: F, t951: F, t959: F, t2924: F, t942: F, t2929: F, t2932: F, t2262: F, t338: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t2937, t2939, t2940) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk794::<F>(t2936, t300, t2898, t938);
        let (t2942, t2944, t2946, t2948, t2950, t2952, t2954, t2955) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk795::<F>(t2940, t961, t2904, t2906, t951, t959, t2924, t942, t2929, t2932, t2262, t338);
    (t2937, t2939, t2940, t2942, t2944, t2946, t2948, t2950, t2952, t2954, t2955)
}
