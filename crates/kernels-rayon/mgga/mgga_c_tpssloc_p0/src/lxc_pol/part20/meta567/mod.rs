//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta567 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2126;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2127;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta567(t1030: f64, t10477: f64, t10472: f64, t10475: f64, t3128: f64, t10903: f64, t10948: f64, t10890: f64, t10898: f64, t3103: f64, t10904: f64, t11002: f64, t10508: f64, t248: f64, t3130: f64, t3132: f64, t10969: f64, t121: f64, t10305: f64, t1041: f64, t1015: f64, t3033: f64, t42520: f64, t3142: f64, t698: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42559, t42561, t42565, t42570, t42573, t42578, t42582) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2126(t1030, t10477, t10472, t10475, t3128, t10903, t10948, t10890, t10898, t3103, t10904, t11002);
        let (t42586, t42595, t42600, t42610) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2127(t10508, t248, t3130, t3132, t10969, t121, t10305, t1041, t1015, t3033, t42520, t3142, t698, t973);
    (t42559, t42561, t42565, t42570, t42573, t42578, t42582, t42586, t42595, t42600, t42610)
}
