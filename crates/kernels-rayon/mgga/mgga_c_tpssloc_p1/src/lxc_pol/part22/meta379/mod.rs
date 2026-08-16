//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta379 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1639;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1640;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta379(t17766: f64, t17798: f64, t17852: f64, t17873: f64, t225: f64, t68: f64, t369: f64, t10457: f64, t248: f64, t5677: f64, t1041: f64, t1044: f64, t17187: f64, t14084: f64, t14085: f64, t14117: f64, t14508: f64, t14511: f64, t1622: f64, t17734: f64, t17738: f64, t3048: f64, t3117: f64, t3130: f64, t378: f64, t4596: f64, t4600: f64, t4636: f64, t4644: f64, t5857: f64, t5861: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17875, t17876, t17877, t17878, t17884, t17885, t17890) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1639(t17766, t17798, t17852, t17873, t225, t68, t369, t10457, t248, t5677, t1041, t1044, t17187);
        let t17900 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1640(t1041, t14084, t14085, t14117, t14508, t14511, t1622, t17734, t17738, t17878, t17885, t17890, t3048, t3117, t3130, t378, t4596, t4600, t4636, t4644, t5857, t5861, t973);
    (t17875, t17876, t17877, t17878, t17884, t17885, t17890, t17900)
}
