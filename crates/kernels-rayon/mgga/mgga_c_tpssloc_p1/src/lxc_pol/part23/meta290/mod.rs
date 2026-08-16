//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta290 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1004;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1005;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1006;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta290(t10784: f64, t10785: f64, t21120: f64, t21124: f64, t21128: f64, t21132: f64, t21136: f64, t21140: f64, t21142: f64, t21144: f64, t21147: f64, t21150: f64, t21153: f64, t21156: f64, t21126: f64, t908: f64, t136: f64, t21122: f64, t2826: f64, t10577: f64, t13598: f64, t17149: f64, t17165: f64, t17175: f64, t894: f64, t901: f64, t1547: f64, t5698: f64, t10599: f64, t10595: f64, t13642: f64, t17286: f64, t17288: f64, t17290: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t21158 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1004(t10784, t10785, t21120, t21124, t21128, t21132, t21136, t21140, t21142, t21144, t21147, t21150, t21153, t21156);
        let (t21160, t21161, t21167, t21168, t21180) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1005(t21126, t908, t136, t21122, t2826, t10577, t13598, t17149, t17165, t17175, t21124, t21128, t21147, t21150, t21153, t21156);
        let (t21181, t21183, t21186, t21188, t21193) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1006(t21180, t894, t901, t1547, t5698, t10599, t10595, t13598, t13642, t17149, t17165, t17175, t17286, t17288, t17290, t21161, t21168);
    (t21158, t21160, t21161, t21167, t21168, t21180, t21181, t21183, t21186, t21188, t21193)
}
