//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta633 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2417;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2418;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta633(t2578: f64, t41189: f64, t9546: f64, t9555: f64, t2573: f64, t41008: f64, t2566: f64, t2570: f64, t9551: f64, t2588: f64, t40341: f64, t207: f64, t215: f64, t39933: f64, t40344: f64, t795: f64, t116: f64, t786: f64, t9534: f64, t133: f64, t6600: f64, t776: f64, t39568: f64, t761: f64, t2535: f64, t9716: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41190, t41192, t41194, t41196, t41197, t41200, t41209) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2417(t2578, t41189, t9546, t9555, t2573, t41008, t2566, t2570, t9551, t2588, t40341, t207, t215, t39933);
        let (t41212, t41214, t41217, t41254, t41255) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2418(t207, t40344, t795, t116, t786, t9534, t133, t6600, t776, t39568, t761, t2535, t9716);
    (t41190, t41192, t41194, t41196, t41197, t41200, t41209, t41212, t41214, t41217, t41254, t41255)
}
