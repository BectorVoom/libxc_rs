//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta557 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2059;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2060;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta557(t116: f64, t786: f64, t9534: f64, t133: f64, t6600: f64, t776: f64, t39568: f64, t761: f64, t39382: f64, t2531: f64, t9713: f64, t39302: f64, t31: f64, t717: f64, t607: f64, t707: f64, t9862: f64, t2617: f64, t9670: f64, t9973: f64, t236: f64, t40931: f64, t10021: f64, t812: f64, t815: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41214, t41217, t41254, t41258, t41259, t41262) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2059(t116, t786, t9534, t133, t6600, t776, t39568, t761, t39382, t2531, t9713, t39302);
        let (t41284, t41291, t41340, t41344, t41347, t41362) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2060(t31, t717, t607, t707, t9862, t2617, t9670, t9973, t236, t40931, t10021, t812, t815);
    (t41214, t41217, t41254, t41258, t41259, t41262, t41284, t41291, t41340, t41344, t41347, t41362)
}
