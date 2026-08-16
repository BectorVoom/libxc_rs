//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta486 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1904;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1905;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1906;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta486(t21158: f64, t21193: f64, t932: f64, t10813: f64, t21114: f64, t21089: f64, t2932: f64, t10542: f64, t10545: f64, t21120: f64, t21124: f64, t21128: f64, t21132: f64, t21136: f64, t21140: f64, t21142: f64, t21144: f64, t21147: f64, t21150: f64, t21153: f64, t21156: f64, t13598: f64, t13642: f64, t17149: f64, t17165: f64, t17175: f64, t17286: f64, t17288: f64, t17290: f64, t21161: f64, t21168: f64, t21181: f64, t21183: f64, t21186: f64, t21188: f64) -> (f64, f64, f64, f64, f64) {
        let (t21194, t21195, t21198, t21207, t21222) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1904(t21158, t21193, t932, t10813, t21114, t21089, t2932, t10542, t10545, t21120, t21124, t21128, t21132, t21136, t21140, t21142, t21144, t21147, t21150, t21153, t21156);
        let t21237 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1905(t13598, t13642, t17149, t17165, t17175, t17286, t17288, t17290, t21161, t21168, t21181, t21183, t21186, t21188);
        let t21238 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1906(t21222, t21237);
    (t21194, t21195, t21198, t21207, t21238)
}
