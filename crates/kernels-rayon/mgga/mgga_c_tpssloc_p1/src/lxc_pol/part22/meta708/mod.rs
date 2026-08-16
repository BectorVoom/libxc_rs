//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta708 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2302;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2303;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2304;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta708(t52: f64, t12874: f64, t12877: f64, t16558: f64, t16563: f64, t17635: f64, t20217: f64, t20234: f64, t2440: f64, t3966: f64, t40647: f64, t4087: f64, t5398: f64, t607: f64, t67060: f64, t76: f64, zeta_threshold: f64, t67064: f64, t157: f64, t182: f64, t46130: f64, t57887: f64, t46132: f64, t46134: f64, t57897: f64, t40667: f64, t40682: f64, t172: f64, t20742: f64, t763: f64, t39309: f64, t39312: f64, t39316: f64, t39320: f64, t40673: f64, t40679: f64, t40685: f64, t16693: f64, t16713: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t67082 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2302(t52, t12874, t12877, t16558, t16563, t17635, t20217, t20234, t2440, t3966, t40647, t4087, t5398, t607, t67060, t76, zeta_threshold);
        let (t67083, t67086, t67087, t67088, t67089, t67090, t67095, t67096, t67097, t67099) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2303(t67064, t67082, t157, t182, t46130, t57887, t46132, t46134, t57897, t40667, t40682, t172, t20742, t763);
        let (t67100, t67101, t67104) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2304(t67099, t39309, t39312, t39316, t39320, t40673, t40679, t40685, t67095, t67096, t67097, t16693, t16713);
    (t67083, t67086, t67087, t67088, t67089, t67090, t67095, t67096, t67097, t67100, t67101, t67104)
}
