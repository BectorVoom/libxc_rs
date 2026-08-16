//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta606 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2131;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2132;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta606(t50077: f64, t3070: f64, t43198: f64, t4578: f64, t4574: f64, t10510: f64, t4641: f64, t1020: f64, t1616: f64, t248: f64, t43216: f64, t10882: f64, t48569: f64, t3039: f64, t4599: f64, t49850: f64, t10870: f64, t4644: f64, t10875: f64, t10903: f64, t14507: f64, t14651: f64, t3069: f64, t4608: f64, t698: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50078, t50148, t50170, t50175, t50181, t50193) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2131(t50077, t3070, t43198, t4578, t4574, t10510, t4641, t1020, t1616, t248, t43216, t10882, t48569);
        let (t50259, t50263, t50265, t50302, t50324, t50361) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2132(t3039, t4599, t49850, t10870, t4644, t10875, t48569, t10903, t14507, t14651, t3069, t4608, t698, t973);
    (t50078, t50148, t50170, t50175, t50181, t50193, t50259, t50263, t50265, t50302, t50324, t50361)
}
