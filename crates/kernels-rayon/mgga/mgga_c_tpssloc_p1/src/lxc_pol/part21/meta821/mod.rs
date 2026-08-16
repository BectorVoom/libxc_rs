//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta821 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2886;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2887;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2888;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta821(t17271: f64, t2815: f64, t896: f64, t17210: f64, t2807: f64, t13615: f64, t4362: f64, t17215: f64, t17218: f64, t17255: f64, t699: f64, t136: f64, t59730: f64, t908: f64, t59698: f64, t60243: f64, t60245: f64, t60248: f64, t60251: f64, t60254: f64, t60257: f64, t60260: f64, t59696: f64, t2826: f64, t59742: f64, t47787: f64, t59700: f64, t59702: f64, t59704: f64, t59708: f64, t59713: f64, t59717: f64, t59721: f64, t59727: f64, t59732: f64, t59735: f64, t59738: f64, t59744: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t60263, t60265, t60267, t60269, t60271, t60274, t60277) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2886(t17271, t2815, t896, t17210, t2807, t13615, t4362, t17215, t17218, t17255, t699, t136, t59730, t908);
        let t60279 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2887(t59698, t60243, t60245, t60248, t60251, t60254, t60257, t60260, t60263, t60265, t60267, t60269, t60271, t60274, t60277);
        let (t60282, t60296, t60300) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2888(t136, t59696, t908, t2826, t59742, t47787, t59700, t59702, t59704, t59708, t59713, t59717, t59721, t59727, t59732, t59735, t59738, t59744);
    (t60263, t60265, t60267, t60269, t60271, t60274, t60277, t60279, t60282, t60296, t60300)
}
