//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta645 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2437;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2438;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta645(t10402: f64, t11037: f64, t2402: f64, t973: f64, t999: f64, t1030: f64, t10477: f64, t10472: f64, t10475: f64, t3128: f64, t10903: f64, t10948: f64, t10890: f64, t10508: f64, t248: f64, t3130: f64, t3132: f64, t1015: f64, t3033: f64, t42520: f64, t3142: f64, t698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42546, t42552, t42559, t42561, t42565, t42570) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2437(t10402, t11037, t2402, t973, t999, t1030, t10477, t10472, t10475, t3128, t10903, t10948);
        let (t42573, t42586, t42600, t42610) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2438(t10890, t10948, t10508, t248, t3130, t3132, t1015, t3033, t42520, t3142, t698, t973);
    (t42546, t42552, t42559, t42561, t42565, t42570, t42573, t42586, t42600, t42610)
}
