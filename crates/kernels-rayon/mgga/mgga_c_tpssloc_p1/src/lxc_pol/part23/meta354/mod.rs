//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta354 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1150;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1151;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta354(t207: f64, t40394: f64, t40399: f64, t786: f64, t9580: f64, t2566: f64, t2570: f64, t2588: f64, t40341: f64, t215: f64, t39933: f64, t40344: f64, t795: f64, t116: f64, t9534: f64, t39568: f64, t761: f64, t39382: f64, t39302: f64, t6589: f64, t68: f64, t236: f64, t40931: f64, t240: f64, t812: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41185, t41189, t41196, t41200, t41209, t41212) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1150(t207, t40394, t40399, t786, t9580, t2566, t2570, t2588, t40341, t215, t39933, t40344, t795);
        let (t41214, t41254, t41258, t41262, t41315, t41349) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1151(t116, t786, t9534, t39568, t761, t39382, t39302, t6589, t68, t236, t40931, t240, t812);
    (t41185, t41189, t41196, t41200, t41209, t41212, t41214, t41254, t41258, t41262, t41315, t41349)
}
