//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta556 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2057;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2058;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta556(t207: f64, t40419: f64, t9538: f64, t41083: f64, t789: f64, t154: f64, t1891: f64, t205: f64, t792: f64, t9558: f64, t40394: f64, t40399: f64, t786: f64, t9580: f64, t2578: f64, t2566: f64, t2570: f64, t2588: f64, t40341: f64, t215: f64, t39933: f64, t40344: f64, t795: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41155, t41156, t41160, t41161, t41170, t41185) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2057(t207, t40419, t9538, t41083, t789, t154, t1891, t205, t792, t9558, t40394, t40399);
        let (t41189, t41190, t41196, t41200, t41209, t41212) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2058(t786, t9580, t2578, t2566, t2570, t2588, t40341, t207, t215, t39933, t40344, t795);
    (t41155, t41156, t41160, t41161, t41170, t41185, t41189, t41190, t41196, t41200, t41209, t41212)
}
