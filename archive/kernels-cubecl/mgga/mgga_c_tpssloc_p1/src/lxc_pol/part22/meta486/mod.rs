//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta486 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1904;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1905;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1906;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta486<F: Float>(t21158: F, t21193: F, t932: F, t10813: F, t21114: F, t21089: F, t2932: F, t10542: F, t10545: F, t21120: F, t21124: F, t21128: F, t21132: F, t21136: F, t21140: F, t21142: F, t21144: F, t21147: F, t21150: F, t21153: F, t21156: F, t13598: F, t13642: F, t17149: F, t17165: F, t17175: F, t17286: F, t17288: F, t17290: F, t21161: F, t21168: F, t21181: F, t21183: F, t21186: F, t21188: F) -> (F, F, F, F, F) {
        let (t21194, t21195, t21198, t21207, t21222) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1904::<F>(t21158, t21193, t932, t10813, t21114, t21089, t2932, t10542, t10545, t21120, t21124, t21128, t21132, t21136, t21140, t21142, t21144, t21147, t21150, t21153, t21156);
        let t21237 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1905::<F>(t13598, t13642, t17149, t17165, t17175, t17286, t17288, t17290, t21161, t21168, t21181, t21183, t21186, t21188);
        let t21238 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1906::<F>(t21222, t21237);
    (t21194, t21195, t21198, t21207, t21238)
}
