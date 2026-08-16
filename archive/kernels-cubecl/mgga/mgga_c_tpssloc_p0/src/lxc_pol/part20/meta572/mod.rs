//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta572 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2135;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta572<F: Float>(t10231: F, t10279: F, t973: F, t42308: F, t974: F, t10224: F, t2999: F, t2978: F, t698: F, t2981: F, t10263: F, t2971: F) -> (F, F, F, F, F, F) {
        let (t42858, t42861, t42873, t42875, t42877, t42889) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2135::<F>(t10231, t10279, t973, t42308, t974, t10224, t2999, t2978, t698, t2981, t10263, t2971);
    (t42858, t42861, t42873, t42875, t42877, t42889)
}
