//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta432 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1763;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1764;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1765;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta432(t5427: f64, t608: f64, t5392: f64, t9287: f64, t607: f64, t3966: f64, t3981: f64, t2267: f64, t5398: f64, t16558: f64, t43: f64, t9300: f64, t3990: f64, t2274: f64, t55: f64, t1420: f64, t39: f64, t3991: f64, t3994: f64, t51: f64, t5408: f64, t5411: f64, t5416: f64, t615: f64, t621: f64, t9311: f64, t33: f64, t9321: f64, t2291: f64, t9330: f64, t2298: f64, t4007: f64, t4012: f64, t634: f64, t638: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19363, t19368, t19369, t19372, t19378, t19381, t19390) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1763(t5427, t608, t5392, t9287, t607, t3966, t3981, t2267, t5398, t16558, t43, t9300);
        let (t19391, t19394, t19398, t19401, t19404) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1764(t19390, t607, t3966, t3990, t2274, t5398, t16558, t55, t1420, t19369, t19372, t19378, t19381, t39, t3991, t3994, t51, t5408, t5411, t5416, t615, t621, t9311);
        let (t19405, t19420, t19430, t19440) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1765(t19404, t33, t5392, t9321, t2291, t5398, t9330, t2298, t16558, t3966, t4007, t4012, t607, t634, t638);
    (t19363, t19368, t19390, t19391, t19394, t19398, t19401, t19404, t19405, t19420, t19430, t19440)
}
