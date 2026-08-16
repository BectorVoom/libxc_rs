//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta572 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2286;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2287;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta572(t5427: f64, t608: f64, t5392: f64, t9287: f64, t607: f64, t3966: f64, t3981: f64, t2267: f64, t5398: f64, t16558: f64, t43: f64, t9300: f64, t3990: f64, t2274: f64, t55: f64, t1420: f64, t39: f64, t3991: f64, t3994: f64, t51: f64, t5408: f64, t5411: f64, t5416: f64, t615: f64, t621: f64, t9311: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19363, t19368, t19369, t19372, t19377, t19378, t19381, t19390) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2286(t5427, t608, t5392, t9287, t607, t3966, t3981, t2267, t5398, t16558, t43, t9300);
        let (t19397, t19404) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2287(t19390, t607, t3966, t3990, t2274, t5398, t16558, t55, t1420, t19369, t19372, t19378, t19381, t39, t3991, t3994, t51, t5408, t5411, t5416, t615, t621, t9311);
    (t19363, t19368, t19369, t19372, t19377, t19378, t19381, t19390, t19397, t19404)
}
