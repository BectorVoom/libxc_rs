//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta549 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2093;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2094;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta549(t13012: f64, t9566: f64, t207: f64, t215: f64, t39933: f64, t40344: f64, t795: f64, t116: f64, t786: f64, t9534: f64, t133: f64, t6600: f64, t776: f64, t2639: f64, t9960: f64, t2427: f64, t9909: f64, t39568: f64, t761: f64, t2535: f64, t9716: f64, t39382: f64, t2531: f64, t9713: f64, t39302: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41205, t41209, t41212, t41214, t41217) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2093(t13012, t9566, t207, t215, t39933, t40344, t795, t116, t786, t9534, t133, t6600, t776);
        let (t41237, t41251, t41254, t41255, t41258, t41259, t41262) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2094(t2639, t9960, t2427, t9909, t39568, t761, t2535, t9716, t39382, t2531, t9713, t39302);
    (t41205, t41209, t41212, t41214, t41217, t41237, t41251, t41254, t41255, t41258, t41259, t41262)
}
