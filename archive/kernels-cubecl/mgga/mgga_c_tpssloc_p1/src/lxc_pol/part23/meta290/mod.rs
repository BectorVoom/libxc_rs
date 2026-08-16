//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta290 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1004;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1005;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1006;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta290<F: Float>(t10784: F, t10785: F, t21120: F, t21124: F, t21128: F, t21132: F, t21136: F, t21140: F, t21142: F, t21144: F, t21147: F, t21150: F, t21153: F, t21156: F, t21126: F, t908: F, t136: F, t21122: F, t2826: F, t10577: F, t13598: F, t17149: F, t17165: F, t17175: F, t894: F, t901: F, t1547: F, t5698: F, t10599: F, t10595: F, t13642: F, t17286: F, t17288: F, t17290: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t21158 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1004::<F>(t10784, t10785, t21120, t21124, t21128, t21132, t21136, t21140, t21142, t21144, t21147, t21150, t21153, t21156);
        let (t21160, t21161, t21167, t21168, t21180) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1005::<F>(t21126, t908, t136, t21122, t2826, t10577, t13598, t17149, t17165, t17175, t21124, t21128, t21147, t21150, t21153, t21156);
        let (t21181, t21183, t21186, t21188, t21193) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1006::<F>(t21180, t894, t901, t1547, t5698, t10599, t10595, t13598, t13642, t17149, t17165, t17175, t17286, t17288, t17290, t21161, t21168);
    (t21158, t21160, t21161, t21167, t21168, t21180, t21181, t21183, t21186, t21188, t21193)
}
