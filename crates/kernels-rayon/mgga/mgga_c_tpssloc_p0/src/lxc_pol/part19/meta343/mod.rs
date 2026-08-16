//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta343 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1228;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1229;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta343(t2427: f64, t9909: f64, t39568: f64, t761: f64, t2535: f64, t9716: f64, t39382: f64, t2531: f64, t9713: f64, t39302: f64, t39563: f64, t39585: f64, t39590: f64, t39593: f64, t40818: f64, t172: f64, t763: f64, t9915: f64, t184: f64, t4194: f64, t607: f64, t9258: f64, t12939: f64, t2244: f64, t9681: f64, t2371: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41252, t41254, t41256, t41258, t41260, t41262, t41263) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1228(t2427, t9909, t39568, t761, t2535, t9716, t39382, t2531, t9713, t39302, t39563, t39585, t39590, t39593, t40818);
        let (t41266, t41270, t41273, t41274) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1229(t172, t763, t9915, t184, t4194, t607, t9258, t12939, t2244, t9681, t2371, t9716);
    (t41252, t41254, t41256, t41258, t41260, t41262, t41263, t41266, t41270, t41273, t41274)
}
