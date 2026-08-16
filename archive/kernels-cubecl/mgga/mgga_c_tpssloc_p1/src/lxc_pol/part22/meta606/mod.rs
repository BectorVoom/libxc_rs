//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta606 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2131;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2132;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta606<F: Float>(t50077: F, t3070: F, t43198: F, t4578: F, t4574: F, t10510: F, t4641: F, t1020: F, t1616: F, t248: F, t43216: F, t10882: F, t48569: F, t3039: F, t4599: F, t49850: F, t10870: F, t4644: F, t10875: F, t10903: F, t14507: F, t14651: F, t3069: F, t4608: F, t698: F, t973: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t50078, t50148, t50170, t50175, t50181, t50193) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2131::<F>(t50077, t3070, t43198, t4578, t4574, t10510, t4641, t1020, t1616, t248, t43216, t10882, t48569);
        let (t50259, t50263, t50265, t50302, t50324, t50361) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2132::<F>(t3039, t4599, t49850, t10870, t4644, t10875, t48569, t10903, t14507, t14651, t3069, t4608, t698, t973);
    (t50078, t50148, t50170, t50175, t50181, t50193, t50259, t50263, t50265, t50302, t50324, t50361)
}
