//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta548 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2091;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2092;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta548(t118: f64, t2576: f64, t794: f64, t9516: f64, t207: f64, t40394: f64, t40399: f64, t2582: f64, t9541: f64, t786: f64, t9580: f64, t2578: f64, t9546: f64, t9555: f64, t2573: f64, t41008: f64, t2566: f64, t2570: f64, t9551: f64, t2588: f64, t40341: f64, t12998: f64, t2553: f64, t686: f64, t9524: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41181, t41185, t41187, t41189, t41190) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2091(t118, t2576, t794, t9516, t207, t40394, t40399, t2582, t9541, t786, t9580, t2578);
        let (t41192, t41194, t41197, t41200, t41203) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2092(t9546, t9555, t2573, t41008, t2566, t2570, t9551, t2588, t40341, t12998, t2553, t686, t9524);
    (t41181, t41185, t41187, t41189, t41190, t41192, t41194, t41197, t41200, t41203)
}
