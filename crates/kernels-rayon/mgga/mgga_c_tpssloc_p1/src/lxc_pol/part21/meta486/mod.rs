//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta486 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2086;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2087;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta486(t16710: f64, t758: f64, t3966: f64, t4195: f64, t4194: f64, t184: f64, t5392: f64, t607: f64, t12939: f64, t13121: f64, t16699: f64, t16700: f64, t16703: f64, t16705: f64, t16707: f64, t16708: f64, t16709: f64, t9853: f64, t9859: f64, t9894: f64, t9907: f64, t9921: f64, t16684: f64, t16686: f64, t16698: f64, t225: f64, t1504: f64, t68: f64, t1891: f64, t5527: f64, t776: f64, t4119: f64, t4226: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16712, t16713, t16715, t16716, t16717, t16719, t16720) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2086(t16710, t758, t3966, t4195, t4194, t184, t5392, t607, t12939, t13121, t16699, t16700, t16703, t16705, t16707, t16708, t16709, t9853, t9859, t9894, t9907, t9921);
        let (t16723, t16729, t16736, t16737, t16740) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2087(t16684, t16686, t16698, t16720, t225, t1504, t68, t1891, t5527, t776, t4119, t4226);
    (t16712, t16713, t16715, t16716, t16717, t16719, t16723, t16729, t16736, t16737, t16740)
}
